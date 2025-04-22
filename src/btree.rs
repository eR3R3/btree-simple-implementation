use std::collections::HashMap;
use std::sync::Arc;
use crate::global::{AtomicPageId, PageId, SchemaRef};

// is_empty	判断是否首次插入
// start_new_tree	初始化一个新的树，创建 root（是叶子）
// find_leaf_page	找到插入的叶子节点
// insert	插入到叶子页
// split	如果页满了就分裂
// insert_internalkv	把新键插入父节点
// new_page + new_root	如果 root 分裂，建一个新的 root

#[derive(Debug)]
pub struct BPlusTreeIndex {
    pub key_schema: SchemaRef,
    pub internal_max_size: u32,
    pub leaf_max_size: u32,
    pub root_page_id: AtomicPageId,
}

impl BPlusTreeIndex {

}