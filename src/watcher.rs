use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};

pub struct FileWatcher {
    rx: mpsc::Receiver<()>,
    _debouncer: notify_debouncer_mini::Debouncer<notify::RecommendedWatcher>,
}

impl FileWatcher {
    pub fn new(path: &Path) -> Option<Self> {
        let (tx, rx) = mpsc::channel();
        let sender = tx.clone();

        let mut debouncer = new_debouncer(Duration::from_millis(300), move |res: Result<Vec<notify_debouncer_mini::DebouncedEvent>, notify::Error>| {
            if let Ok(events) = res {
                for event in events {
                    if event.kind == DebouncedEventKind::Any {
                        let _ = sender.send(());
                        break;
                    }
                }
            }
        }).ok()?;

        debouncer
            .watcher()
            .watch(path, notify::RecursiveMode::NonRecursive)
            .ok()?;

        Some(Self {
            rx,
            _debouncer: debouncer,
        })
    }

    /// Check if the file has changed (non-blocking)
    pub fn has_changed(&self) -> bool {
        self.rx.try_recv().is_ok()
    }
}
