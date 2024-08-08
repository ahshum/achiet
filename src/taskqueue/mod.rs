mod dispatcher;
mod task;
mod worker;

pub use dispatcher::*;
pub use task::*;
pub use worker::*;

pub fn channel() -> (dispatcher::Dispatcher, worker::Worker) {
    let (sx, rx) = crossbeam::channel::unbounded();
    (dispatcher::Dispatcher::new(sx), worker::Worker::new(rx))
}
