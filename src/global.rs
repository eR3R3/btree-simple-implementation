use std::sync::Arc;
use crate::column::{Column};
use crate::schema::Schema;
use crate::tuple::Tuple;

pub type PageId = usize;
pub type RecordId = usize;

pub type SchemaRef = Arc<Schema>;

pub type ColumnRef = Arc<Column>;

pub type InternalKV = (Tuple, PageId);

pub type LeafKV = (Tuple, RecordId);

pub const INVALID_PAGE_ID: PageId = 0;