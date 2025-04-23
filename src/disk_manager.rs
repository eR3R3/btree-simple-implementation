use std::fs::File;
use std::sync::atomic::AtomicU32;
use std::sync::{Mutex, RwLock};
use crate::page::meta_page::MetaPage;

#[derive(Debug)]
pub struct DiskManager {
    next_page_id: AtomicU32,
    db_file: Mutex<File>,
    pub meta: RwLock<MetaPage>,
}