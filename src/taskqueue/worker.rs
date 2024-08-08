use super::task::Task;
use crate::app::AppState;

#[derive(Clone)]
pub struct Worker {
    rx: crossbeam::channel::Receiver<Task>,
}

impl Worker {
    pub fn new(rx: crossbeam::channel::Receiver<Task>) -> Self {
        Self { rx }
    }

    pub async fn work(&self, app_state: AppState) -> Result<(), ()> {
        loop {
            let _ = match self.rx.recv() {
                Ok(task) => task.run(&app_state).await,
                Err(_) => return Err(()),
            };
        }
    }
}
