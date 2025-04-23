use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use dashmap::DashMap;
use crate::global::{FrameId, PageId, PAGE_SIZE};
use crate::buffer::frame::Frame;
use crate::buffer::replacer::LRUKReplacer;
use crate::disk_manager::DiskManager;

#[derive(Debug)]
pub struct BufferPoolManager {
    // This thing right here is not thread safe, so the only thing we can do is to
    // read a single item and operate that item, we cannot have two threads pushing
    // at the same time
    pool: Vec<Arc<RwLock<Frame>>>,
    // LRU-K置换算法
    pub replacer: Arc<RwLock<LRUKReplacer>>,
    pub disk_manager: Arc<DiskManager>,
    // 缓冲池中的页号与frame号的映射
    page_table: Arc<DashMap<PageId, FrameId>>,
    // the empty frame positions in buffer pool, this is different from the free_list page on disk,
    // this one on memory is recording the free frame in the buffer pool that is
    // not pinned
    free_list: Arc<RwLock<VecDeque<FrameId>>>,
}

