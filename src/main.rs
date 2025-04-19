use btree_simple_implementation::global::PageId;
use btree_simple_implementation::page::BPlusTreePage;

pub struct BPlusTree {
    pub root_page_id: PageId,
    pub pages: std::collections::HashMap<PageId, BPlusTreePage>,
    pub next_page_id: PageId,
}

fn main() {

}
