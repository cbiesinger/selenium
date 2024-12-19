use crate::logger::Logger;
use anyhow::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::files::{create_parent_path_if_not_exists, create_path_if_not_exists};
use fs2::FileExt;
use std::fs;

const LOCK_FILE: &str = "sm.lock";

pub struct Lock {
    file: File,
    path: PathBuf,
}

impl Lock {
    // Acquire file lock to prevent race conditions accessing the cache folder by concurrent SM processes
    pub fn acquire(
        log: &Logger,
        target: &Path,
        single_file: Option<String>,
    ) -> Result<Self, Error> {
        let lock_folder = if single_file.is_some() {
            create_parent_path_if_not_exists(target)?;
            target.parent().unwrap()
        } else {
            create_path_if_not_exists(target)?;
            target
        };
        let path = lock_folder.join(LOCK_FILE);
        let file = File::create(&path)?;

        log.debug(format!("Acquiring lock: {}", path.display()));
        file.lock_exclusive().unwrap_or_default();

        Ok(Self { file, path })
    }

    pub fn release(&mut self) {
        fs::remove_file(&self.path).unwrap_or_default();
        self.file.unlock().unwrap_or_default();
    }

    pub fn exists(&mut self) -> bool {
        self.path.exists()
    }
}
