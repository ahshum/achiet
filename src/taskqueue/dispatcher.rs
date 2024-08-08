use super::{task::Task, worker::Worker};

#[derive(Clone)]
pub struct Dispatcher {
    sx: crossbeam::channel::Sender<Task>,
}

impl Dispatcher {
    pub fn new(sx: crossbeam::channel::Sender<Task>) -> Self {
        Self { sx }
    }

    pub fn dispatch(&self, task: Task) -> Result<(), ()> {
        self.sx.send(task).map_err(|_| ())
    }
}
