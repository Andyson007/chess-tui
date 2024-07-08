//! Responsible for handling the stockfish instance
use std::{
    io::{BufRead, BufReader, Read, Write},
    process::{Command, Stdio},
    sync::Arc,
    thread::{self, JoinHandle},
};

mod eval;
mod options;
mod thread_stuff;

use eval::Eval;
use thread_stuff::Wait;

use crossbeam::atomic::AtomicCell;
use futures::{
    channel::mpsc::{self, Sender, UnboundedReceiver},
    executor::block_on,
    SinkExt,
};

#[derive(Debug)]
/// The main stockfish instance
pub struct Stockfish {
    handle: JoinHandle<()>,
    sender: Sender<Action>,
    receiver: UnboundedReceiver<String>,
    eval: Arc<Wait<AtomicCell<Eval>>>,
}

impl Stockfish {
    /// Spawn a new thread to handle a stockfish instance. This thread can be sent commands
    /// to with to `Action` enum.
    /// You can get data from it by reading from the receiver
    /// # Panics
    /// Something failed ig
    #[must_use]
    pub fn new() -> Self {
        let (sync_sender, mut thread_receiver) = mpsc::channel::<Action>(8);
        let (mut thread_sender, sync_receiver) = mpsc::unbounded::<String>();
        let eval = Arc::new(Wait::new(AtomicCell::default()));
        let thread_eval = Arc::clone(&eval);
        let handle = thread::spawn(move || {
            futures::executor::block_on(async {
                let mut instance = Command::new("stockfish")
                    .stdin(Stdio::piped())
                    .stderr(Stdio::null())
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();
                let handle = instance.stdin.as_mut().unwrap();
                let stdout = instance.stdout.as_mut().unwrap();
                let mut buf = BufReader::new(stdout);
                let mut reader_buf = String::new();
                buf.read_line(&mut reader_buf).unwrap();
                reader_buf.clear();
                handle.write_all(b"uci\n").unwrap();
                for _ in 0..3 {
                    buf.read_line(&mut reader_buf).unwrap();
                    eprintln!("a{reader_buf}a");
                    reader_buf.clear();
                }
                // TODO: Parse all the options before sending a ucinewgame
                let mut options = Vec::new();
                while buf.read_line(&mut reader_buf).is_ok() {
                    if reader_buf == *"uciok\n" {
                        break;
                    }
                    options.push(options::Option::parse(&reader_buf));
                    reader_buf.clear();
                }
                eprintln!("{options:?}");
                loop {
                    match thread_receiver.try_next() {
                        Ok(Some(x)) => match x {
                            Action::SetFen(x) => handle
                                .write_all(format!("position fen {x}\n").as_bytes())
                                .unwrap(),
                            Action::Start => handle.write_all(b"go\n").unwrap(),
                            Action::Stop => {
                                handle.write_all(b"stop\n").unwrap();
                                buf.read_line(&mut reader_buf).unwrap();
                            }
                            Action::Eval => {
                                handle.write_all(b"eval\n").unwrap();
                                // HACK: This shouldn't just read 72 lines but idk how to improve
                                for _ in 0..72 {
                                    let _ = buf.read_line(&mut reader_buf);
                                    eprintln!("{reader_buf}");
                                }
                                thread_eval.data.store(Eval::parse(&reader_buf));
                                thread_eval.stop_waiting();
                            }
                        },
                        // `Ok(None)` when channel is closed and no messages left in the queue
                        Ok(None) => return,
                        // `Err(e)` when there are no messages available, but channel is not yet closed
                        Err(_) => (),
                    }
                }
            });
        });

        Self {
            handle,
            receiver: sync_receiver,
            sender: sync_sender,
            eval,
        }
    }

    /// Sets the fen on the board to analyze
    pub fn set_fen(&mut self, fen: String) {
        let _ = block_on(self.sender.send(Action::SetFen(fen)));
    }

    /// Makes stockfish start analyzing the position
    pub fn start(&mut self) {
        let _ = block_on(self.sender.send(Action::Start));
    }

    /// Makes stockfish stop analyzing the position
    pub fn stop(&mut self) {
        let _ = block_on(self.sender.send(Action::Stop));
    }

    /// Makes the stockfish instance run eval and returns
    /// a parsed version of that
    pub fn get_eval(&mut self) -> Eval {
        // let eval = &mut self.eval;
        let eval = &*self.eval;
        eval.set_waiting();
        let _ = block_on(self.sender.send(Action::Eval));
        eprintln!("before waiting");
        eval.wait().load()
    }
}

impl Default for Stockfish {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::stockfish::Stockfish;

    #[test]
    fn stockfish() {
        let mut stockfish = Stockfish::new();
        stockfish.set_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
        stockfish.start();
        stockfish.stop();
        let eval = stockfish.get_eval();
        eprintln!("{eval:?}");
    }
}

#[derive(Debug)]
enum Action {
    SetFen(String),
    Start,
    Stop,
    Eval,
}
