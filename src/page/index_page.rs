use std::cmp::Ordering::Equal;
use std::sync::Arc;
use crate::global::{InternalKV, LeafKV, PageId, RecordId, SchemaRef, INVALID_PAGE_ID};
use anyhow::{bail, Result};
use crate::table_component::schema::Schema;
use crate::table_component::tuple::Tuple;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BPlusTreePageType {
    LeafPage,
    InternalPage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BPlusTreePage {
    Leaf(LeafPage),
    Internal(InternalPage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeafPageHeader {
    pub next_page_id: PageId,
    pub current_size: usize,
    pub page_type: BPlusTreePageType,
    pub max_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeafPage {
    pub schema: SchemaRef,
    pub header: LeafPageHeader,
    pub tuples: Vec<(Tuple, RecordId)>,
}

impl LeafPage {
    pub fn new(schema: SchemaRef, max_size: usize) -> Self {
        Self {
            schema,
            header: LeafPageHeader {
                page_type: BPlusTreePageType::LeafPage,
                current_size: 0,
                max_size,
                next_page_id: INVALID_PAGE_ID,
            },
            tuples: Vec::with_capacity(max_size as usize),
        }
    }

    pub fn empty() -> Self {
        Self {
            schema: Arc::new(Schema::empty()),
            header: LeafPageHeader {
                page_type: BPlusTreePageType::LeafPage,
                current_size: 0,
                max_size: 0,
                next_page_id: INVALID_PAGE_ID,
            },
            tuples: Vec::new(),
        }
    }

    pub fn min_size(&self) -> usize {
        self.header.max_size / 2
    }

    pub fn key_at(&self, index: usize) -> &Tuple {
        &self.tuples[index].0
    }

    pub fn kv_at(&self, index: usize) -> &LeafKV {
        &self.tuples[index]
    }

    pub fn is_full(&self) -> bool {
        self.header.current_size > self.header.max_size
    }

    pub fn insert(&mut self, key: Tuple, rid: RecordId) {
        self.tuples.push((key, rid));
        self.header.current_size += 1;
        self.tuples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }

    pub fn batch_insert(&mut self, kvs: Vec<LeafKV>) {
        let kvs_len = kvs.len();
        self.tuples.extend(kvs);
        self.header.current_size += kvs_len;
        self.tuples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }

    pub fn split_off(&mut self, at: usize) -> Vec<LeafKV> {
        let new_array = self.tuples.split_off(at);
        self.header.current_size -= new_array.len();
        new_array
    }

    pub fn reverse_split_off(&mut self, at: usize) -> Vec<LeafKV> {
        let mut new_array = Vec::new();
        for _ in 0..=at {
            new_array.push(self.tuples.remove(0));
        }
        self.header.current_size -= new_array.len();
        new_array
    }

    pub fn delete(&mut self, key: &Tuple) {
        let key_index = self.key_index(key);
        if let Some(index) = key_index {
            self.tuples.remove(index);
            self.header.current_size -= 1;
        }
    }

    // 查找key对应的rid
    pub fn look_up(&self, key: &Tuple) -> Option<RecordId> {
        let key_index = self.key_index(key);
        key_index.map(|index| self.tuples[index].1)
    }

    fn key_index(&self, key: &Tuple) -> Option<usize> {
        if self.header.current_size == 0 {
            return None;
        }
        let mut start: i32 = 0;
        let mut end: i32 = self.header.current_size as i32 - 1;
        while start < end {
            let mid = (start + end) / 2;
            let compare_res = key.partial_cmp(&self.tuples[mid as usize].0).unwrap();
            if compare_res == std::cmp::Ordering::Equal {
                return Some(mid as usize);
            } else if compare_res == std::cmp::Ordering::Less {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        if key.partial_cmp(&self.tuples[start as usize].0).unwrap() == std::cmp::Ordering::Equal {
            return Some(start as usize);
        }
        None
    }

    pub fn next_closest(&self, tuple: &Tuple, included: bool) -> Option<usize> {
        for (idx, (key, _)) in self.tuples.iter().enumerate() {
            if tuple == key && included {
                return Some(idx);
            }
            if key > tuple {
                return Some(idx);
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternalPageHeader {
    pub page_type: BPlusTreePageType,
    pub current_size: usize,
    pub max_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternalPage {
    schema: Schema,
    header: InternalPageHeader,
    tuples: Vec<(Tuple, PageId)>
}

impl InternalPage {
    pub fn new(schema: Schema, max_size: usize) -> Self {
        let header =  InternalPageHeader {
            page_type: BPlusTreePageType::InternalPage,
            current_size: 0,
            max_size,
        };

        Self {
            schema,
            header,
            tuples: Vec::with_capacity(max_size),
        }
    }

    pub fn min_size(&self) -> usize { self.header.max_size / 2 }
    pub fn get_key(&self, index: usize) -> &Tuple {
        &self.tuples[index].0
    }
    pub fn get_value(&self, index: usize) -> PageId {
        self.tuples[index].1
    }

    // the sibling_page_id function is querying about the sibling pages of the page with page_id
    // provided, but the page with page_id provided must be one of the child page of the current
    // internal page
    pub fn sibling_page_id(&self, page_id: PageId) -> (Option<usize>, Option<usize>) {
        let mut left_page = None;
        let mut right_page = None;
        let position = self.tuples.iter().position(|entry| { entry.1 == page_id });
        if let Some(index) = position {
            // if the page found is no the leftest child, it has a left sibling
            if index != 0 {
                left_page = Some(self.tuples[index - 1].1);
            }
            // if the page found is not the rightest child, it has a right sibling
            if index != (self.tuples.len() - 1) {
                right_page = Some(self.tuples[index + 1].1);
            }
        }
        (left_page, right_page)
    }

    fn insert(&mut self, key: Tuple, page_id: PageId) {
        self.tuples.push((key, page_id));
        self.header.current_size += 1;
        let null_kv = self.tuples.remove(0);
        self.tuples.sort_by(|x, y| { x.0.partial_cmp(&y.0).unwrap() });
        self.tuples.insert(0, null_kv);
    }

    // this one does not skip the empty key at the start
    pub fn batch_insert(&mut self, kvs: Vec<InternalKV>) {
        let kvs_len = kvs.len();
        self.tuples.extend(kvs);
        self.header.current_size += kvs_len;
        self.tuples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }

    pub fn delete_by_key(&mut self, tuple: &Tuple) -> Result<InternalKV> {
        if let Ok(index) = self.tuples.binary_search_by(|key| {
            return key.0.partial_cmp(&tuple).unwrap_or(Equal); }) {
            let removed_entry = self.tuples[index].clone();
            self.tuples.remove(index);
            self.header.current_size -= 1;
            if self.header.current_size == 1 {
                self.tuples.remove(0);
                self.header.current_size -= 1;
            }
            return Ok(removed_entry)
        } else {
            bail!("key not found");
        };
    }

    pub fn delete_by_page_id(&mut self, page_id: PageId) {
        for i in 0..self.header.current_size {
            if self.tuples[i].1 == page_id {
                self.tuples.remove(i);
                self.header.current_size -= 1;
                return;
            }
        }
    }

    pub fn is_full(&self) -> bool {
        self.header.current_size > self.header.max_size
    }

    pub fn split_off(&mut self, at: usize) -> Vec<InternalKV> {
        let new_array = self.tuples.split_off(at);
        self.header.current_size -= new_array.len();
        new_array
    }

    pub fn reverse_split_off(&mut self, at: usize) -> Vec<InternalKV> {
        let mut new_array = Vec::new();
        for _ in 0..=at {
            new_array.push(self.tuples.remove(0));
        }
        self.header.current_size -= new_array.len();
        new_array
    }

    pub fn replace_key(&mut self, old_key: &Tuple, new_key: Tuple) {
        let key_index = self.key_index(old_key);
        if let Some(index) = key_index {
            self.tuples[index].0 = new_key;
        }
    }

    pub fn key_index(&self, key: &Tuple) -> Option<usize> {
        if let Ok(index) = self.tuples.binary_search_by(|x| {x.0.partial_cmp(key).unwrap()}) {
            return Some(index);
        } else {
            None
        }
    }

    // pub fn look_up(&self, key: &Tuple) -> PageId {
    //     if self.header.current_size == 0 {
    //         println!("look_up empty page");
    //     }
    //     match self.tuples.binary_search_by(|x| { x.0.partial_cmp(key).unwrap() }) {
    //         Ok(index) => self.tuples[index].1,
    //         Err(_) =>
    //     }
    //
    // }

    pub fn look_up(&self, key: &Tuple) -> PageId {
        // 第一个key为空，所以从1开始
        let mut start = 1;
        if self.header.current_size == 0 {
            println!("look_up empty page");
        }
        let mut end = self.header.current_size - 1;
        while start < end {
            let mid = (start + end) / 2;
            let compare_res = key.partial_cmp(&self.tuples[mid].0).unwrap();
            if compare_res == Equal {
                return self.tuples[mid].1;
            } else if compare_res == std::cmp::Ordering::Less {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        let compare_res = key.partial_cmp(&self.tuples[start].0).unwrap();
        if compare_res == std::cmp::Ordering::Less {
            self.tuples[start - 1].1
        } else {
            self.tuples[start].1
        }
    }
}

