use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use crate::table_component::column::Column;
use crate::table_component::schema::Schema;
use crate::table_component::tuple::Tuple;


pub type PageId = usize;
pub type RecordId = usize;

pub type SchemaRef = Arc<Schema>;

pub type ColumnRef = Arc<Column>;

pub type InternalKV = (Tuple, PageId);

pub type LeafKV = (Tuple, RecordId);

pub const INVALID_PAGE_ID: PageId = 0;

pub const PAGE_SIZE: usize = 4096;

pub type AtomicPageId = AtomicU32;

pub type FrameId = usize;

pub const BUFFER_POOL_SIZE: usize = 1000;

static EMPTY_PAGE: [u8; PAGE_SIZE] = [0; PAGE_SIZE];