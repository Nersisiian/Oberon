use std::path::{Path, PathBuf};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{debug, warn};

use crate::core::config::Config;
use crate::Result;

/// Thread-safe sandbox with interior mutability for dry-run mode.
#[derive(Clone)]
pub struct Sandbox {
    config: Arc<Config>,
    dry_run: Arc<RwLock<bool>>,
}

impl Sandbox {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            dry_run: Arc::new(RwLock::new(false)),
        }
    }

    pub fn is_path_allowed(&self, path: &Path) -> bool {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let allowed = self
            .config
            .safety
            .allowlist
            .iter()
            .any(|allowed| canonical.starts_with(allowed));

        if !allowed {
            debug!(?path, "Path not in allowlist");
        }
        allowed
    }

    pub fn check_file_access(&self, path: &Path, write: bool) -> Result<()> {
        if !self.is_path_allowed(path) {
            return Err(crate::OberonError::Safety(format!(
                "Path not in allowlist: {}",
                path.display()
            )));
        }

        if write && self.config.safety.require_confirmation && !*self.dry_run.read() {
            warn!("Destructive action requires confirmation (simulated)");
            // In production, you'd prompt the user here.
        }

        Ok(())
    }

    pub fn set_dry_run(&self, dry_run: bool) {
        *self.dry_run.write() = dry_run;
    }

    pub fn is_dry_run(&self) -> bool {
        *self.dry_run.read()
    }
}