use std::{
    fmt::Debug,
    sync::{Condvar, Mutex},
};

#[derive(Debug)]
pub struct Wait<T>
where
    T: Debug,
{
    pub data: T,
    #[allow(clippy::struct_field_names)]
    should_wait: Mutex<bool>,
    cond: Condvar,
}

impl<T> Wait<T>
where
    T: Debug,
{
    pub fn wait(&self) -> &T {
        let mut should_wait = self.should_wait.lock().unwrap();
        while !*should_wait {
            should_wait = self.cond.wait(should_wait).unwrap();
        }
        drop(should_wait);
        &self.data
    }

    pub fn new(data: T) -> Self {
        Self {
            data,
            should_wait: Mutex::new(false),
            cond: Condvar::new(),
        }
    }
    pub fn set_waiting(&self) {
        **self.should_wait.lock().as_mut().unwrap() = false;
    }
    pub fn stop_waiting(&self) {
        **self.should_wait.lock().as_mut().unwrap() = true;
        self.cond.notify_one();
    }
}
