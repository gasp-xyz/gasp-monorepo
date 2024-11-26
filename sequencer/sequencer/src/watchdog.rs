use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::timeout;
use tokio::time::Duration;
use tracing;

pub struct Watchdog {
    rx: Receiver<()>,
    duration: Duration,
}

impl Watchdog {
    pub fn new(duration: tokio::time::Duration) -> (Sender<()>, Self) {
        let (tx, rx) = channel::<()>(1);
        (tx, Watchdog { rx, duration })
    }

    pub async fn run(&mut self) {
        loop {
            if let None = timeout(self.duration, self.rx.recv())
                .await
                .expect("watchdog timeout")
            {
                tracing::error!("Watchdog closed");
                break;
            }
        }
    }
}
