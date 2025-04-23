use anyhow::{bail, Result};
use std::collections::{HashMap, LinkedList, VecDeque};
use crate::global::FrameId;

// each node corresponds to a frame
#[derive(Debug)]
struct LRUKNode {
    k: usize,
    // 该frame最近k次被访问的时间
    history: VecDeque<u64>,
    // 是否可被置换
    is_evictable: bool,
}

impl LRUKNode {
    pub fn new(k: usize) -> Self {
        Self {
            k,
            history: VecDeque::new(),
            is_evictable: false,
        }
    }
    pub fn record_access(&mut self, timestamp: u64) {
        self.history.push_back(timestamp);
        if self.history.len() > self.k {
            self.history.pop_front();
        }
    }

    pub fn get_history(&self) -> u64 {
        self.history.front().unwrap().clone()
    }

    pub fn has_empty_history(&self) -> bool {
        if self.history.len() == 0 {
            return true;
        }
        false
    }
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

impl LRUKReplacer {
    pub fn new(num_frames: usize, k: usize) -> Self {
        Self {
            current_size: 0,
            replacer_size: num_frames,
            k,
            node_store: HashMap::new(),
            current_timestamp: 0,
        }
    }

    pub fn set_evictable(&mut self, frame_id: FrameId, set_evictable: bool) -> Result<()> {
        if let Some(node) = self.node_store.get_mut(&frame_id) {
            let evictable = node.is_evictable;
            node.is_evictable = set_evictable;
            if set_evictable && !evictable {
                self.current_size += 1;
            } else if !set_evictable && evictable {
                self.current_size -= 1;
            }
            Ok(())
        } else {
            bail!("no frame with frame_id provided found")
        }
    }

    // this function returns the Some(frameId) if it can find anything that is evictable, or it
    // will return None
    pub fn evict(&mut self) -> Option<FrameId> {
        let mut max_k_distance: u64 = 0;
        let mut result = None;
        for (&frameId, node) in self.node_store.iter() {
            if !node.is_evictable {
                continue;
            }
            // if some node has no access history, then just kick that out
            if node.has_empty_history() {
                result = Some(frameId);
                break;
            }
            let mut k_distance;
            // if the record number is fewer than k, we want to kick those out first
            if self.k > node.history.len() {
                k_distance = u64::MAX - node.get_history();
            } else {
                // if the record number is greater or equal to k, we calculate it by current_time - last_time
                k_distance = self.current_timestamp - node.get_history();
            }

            if k_distance > max_k_distance {
                max_k_distance = k_distance;
                result = Some(frameId);
            }
        }
        if let Some(frame_id) = result {
            self.remove(frame_id);
        }
        result
    }

    // this function remove the provided frameId key-value pair in self.node_store, used in evict
    pub fn remove(&mut self, frame_id: FrameId) {
        if let Some(node) = self.node_store.get(&frame_id) {
            assert!(node.is_evictable, "frame is not evictable");
            self.node_store.remove(&frame_id);
            // decrease the evictable frame by 1
            self.current_size -= 1;
        }
    }

    pub fn record_access(&mut self, frame_id: FrameId) -> Result<()> {
        if let Some(node) = self.node_store.get_mut(&frame_id) {
            node.record_access(self.current_timestamp);
            self.current_timestamp += 1;
        } else {
            // if no node with the frame_id
            if self.node_store.len() >= self.replacer_size {
                bail!("frame size exceeds the limit")
            }
            let mut node = LRUKNode::new(self.k);
            node.record_access(self.current_timestamp);
            self.current_timestamp += 1;
            self.node_store.insert(frame_id, node);
        }
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.current_size
    }
}






