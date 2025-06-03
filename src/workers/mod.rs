use crate::AppWindow;
use slint::{ComponentHandle, Weak};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub enum WorkerMessage {
    Quit,
}

pub struct BackgroundWorker {
    pub channel: UnboundedSender<WorkerMessage>,
    worker_thread: std::thread::JoinHandle<()>,
}

impl BackgroundWorker {
    pub fn new(ui: &AppWindow) -> Self {
        let (channel, rx) = tokio::sync::mpsc::unbounded_channel();

        let worker_thread = std::thread::spawn({
            let handle = ui.as_weak();
            move || {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(background_worker_thread(rx, handle))
                    .unwrap()
            }
        });

        Self {
            channel,
            worker_thread,
        }
    }

    pub fn join(self) -> std::thread::Result<()> {
        let _ = self.channel.send(WorkerMessage::Quit);
        self.worker_thread.join()
    }
}

async fn background_worker_thread(
    mut rx: UnboundedReceiver<WorkerMessage>,
    ui_handle: Weak<AppWindow>,
) -> tokio::io::Result<()> {
    loop {
        let message = match rx.recv().await {
            Some(m) => m,
            None => return Ok(()),
        };

        match message {
            WorkerMessage::Quit => return Ok(()),
        }
    }
}
