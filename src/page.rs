use crate::global::{PageId, RecordId};
use crate::schema::Schema;
use crate::tuple::Tuple;

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
    pub next_page_id: Option<PageId>,
    pub current_size: usize,
    pub max_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeafPage {
    pub schema: Schema,
    pub header: LeafPageHeader,
    pub tuples: Vec<(Tuple, RecordId)>,
    pub next_page_id: PageId,
}

impl LeafPage {

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
        let position = self.tuples.iter().position(|tuple: Tuple, each_page_id: PageId| { each_page_id == page_id });
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
        self.tuples.remove(0);
        self.tuples.sort_by(|x, y| {
            x.0.c
        })
    }
}

