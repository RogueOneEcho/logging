//! Global logger initialization.

use crate::{Logger, Verbosity};
use colored::control::SHOULD_COLORIZE;
use colored::Colorize;
use log::{set_boxed_logger, set_max_level, trace, Log};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Register a logger as the global `log` logger.
///
/// Returns `true` if the logger was initialized, or `false` if a logger was
/// already registered. Only the first call has any effect; subsequent calls
/// are no-ops. This makes it safe to call from multiple tests running in
/// parallel.
pub trait InitLog: Log + Sized + 'static {
    fn init(self) -> bool;
}

impl InitLog for Logger {
    fn init(self) -> bool {
        let verbosity = self.options.verbosity;
        init(self, verbosity)
    }
}

impl InitLog for Arc<Logger> {
    fn init(self) -> bool {
        let verbosity = self.options.verbosity;
        init(self, verbosity)
    }
}

/// `swap` atomically reads and sets the flag in a single operation, preventing
/// a race where two threads could both read `false` and both proceed to
/// initialize.
fn init(logger: impl Log + 'static, verbosity: Option<Verbosity>) -> bool {
    if IS_INITIALIZED.swap(true, Ordering::Relaxed) {
        return false;
    }
    SHOULD_COLORIZE.set_override(true);
    match set_boxed_logger(Box::new(logger)) {
        Ok(()) => set_max_level(verbosity.unwrap_or_default().to_level_filter()),
        Err(error) => {
            trace!("{} to initialize the logger: {}", "Failed".bold(), error);
        }
    }
    true
}
