// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

//use named_lock::{NamedLock, NamedLockGuard};
use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

//const PACKAGE_LOCK_NAME: &str = "move_pkg_lock";
static PACKAGE_THREAD_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
//static PACKAGE_PROCESS_MUTEX: Lazy<NamedLock> =
//    Lazy::new(|| NamedLock::create(PACKAGE_LOCK_NAME).unwrap());

/// The package lock is a lock held across threads and processes. This lock is held to ensure that
/// the Move package manager has a consistent (read: serial) view of the file system. Without this
/// lock we can easily get into race conditions around caching and overwriting of packages (e.g.,
/// thread 1 and thread 2 compete to build package P in the same location), as well as downloading
/// of git dependencies (thread 1 starts downloading git dependency, meanwhile thread 2 sees the
/// git directory before it has been fully populated but assumes it has been fully downloaded and
/// starts building the package before the git dependency has been fully downloaded by thread 1.
/// This will then lead to file not found errors). These same issues could occur across processes,
/// this is why we grab both a thread lock and process lock.
pub(crate) struct PackageLock {
    thread_lock: MutexGuard<'static, ()>,
    //process_lock: NamedLockGuard<'static>,
}

impl PackageLock {
    pub(crate) fn lock() -> PackageLock {
        let thread_lock = PACKAGE_THREAD_MUTEX.lock().unwrap();
        //let process_lock = PACKAGE_PROCESS_MUTEX.lock().unwrap();
        Self {
            thread_lock,
            //process_lock,
        }
    }

    pub(crate) fn unlock(self) {
        let Self {
            thread_lock,
            //process_lock,
        } = self;
        //drop(process_lock);
        drop(thread_lock);
    }
}
