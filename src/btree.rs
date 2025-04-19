use std::collections::HashMap;
use crate::global::PageId;
use crate::page::BPlusTreePage;

pub struct BPlusTree {
    pub root_page_id: PageId,
    pub pages: HashMap<PageId, BPlusTreePage>,
    pub next_page_id: PageId,
    pub leaf_max_size: usize,
    pub internal_max_size: usize,
}

impl BPlusTree {
    pub fn new() -> Self {
        let root_page_id = 0;


        Self {
            root_page_id,
            pages: HashMap::new()
        }
    }
}