use std::collections::{HashMap, LinkedList};
use crate::global::FrameId;


#[derive(Debug)]
struct LRUKNode {
    k: usize,
    // 该frame最近k次被访问的时间
    history: LinkedList<u64>,
    // 是否可被置换
    is_evictable: bool,
}

#[derive(Debug)]
pub struct LRUKReplacer {
    // 当前可置换的frame数
    current_size: usize,
    // 可置换的frame数上限
    replacer_size: usize,
    k: usize,
    node_store: HashMap<FrameId, LRUKNode>,
    // 当前时间戳（从0递增）
    current_timestamp: u64,
}