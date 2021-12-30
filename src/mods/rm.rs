use std::sync::Arc;

use walkdir::WalkDir;

use super::base::ByeCacheMod;

#[derive(Default)]
pub struct RmModule {
    pub dir: Arc<String>,
    stop_channel: Option<oneshot::Sender<()>>,
    join_handle: Option<std::thread::JoinHandle<()>>,
}

impl ByeCacheMod for RmModule {
    fn execute(mut self) -> Self {
        let (stop_sender, stop_receiver) = oneshot::channel();
        let dir = self.dir.clone();
        
        self.join_handle = Some(std::thread::spawn(move || {
            rm_core(dir, stop_receiver);
        }));
        self.stop_channel = Some(stop_sender);
        self
    }

    fn wait(mut self) -> Self {
        if let Some(handle) = self.join_handle {
            handle.join().expect("failed to run rm successfully");
        } else {
            eprintln!("warning: rm is not running");
        }

        self.stop_channel = None;
        self.join_handle = None;
        self
    }

    fn stop(mut self) -> Self {
        if let Some(chan) = self.stop_channel {
            chan.send(()).expect("failed to abort rm");
        } else {
            eprintln!("warning: rm is not running");
        }

        self.stop_channel = None;
        self.join_handle = None;
        self
    }
}

/// The core function of `rm`.
fn rm_core(dir: Arc<String>, stop_receiver: oneshot::Receiver<()>) {
    for entry in WalkDir::new(&*dir) {
        match entry {
            Ok(entry) => println!("rm '{}'", entry.path().display()),
            Err(err) => println!("failed to rm: {}", err),
        }

        if stop_receiver.try_recv().is_ok() {
            break;
        }
    }
}

#[derive(Default)]
pub struct RmModuleBuilder {
    dir: Option<String>,
}

impl RmModuleBuilder {
    pub fn new() -> Self {
        RmModuleBuilder::default()
    }

    pub fn dir(mut self, dir: String) -> Self {
        self.dir = Some(dir);
        self
    }

    pub fn build(self) -> RmModule {
        if self.dir.is_none() {
            panic!("-- struct rm: FATAL: dir is not set");
        }
        RmModule {
            dir: Arc::new(self.dir.unwrap_or_else(|| "/".to_string())),
            ..RmModule::default()
        }
    }
}
