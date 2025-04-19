use std::sync::Arc;
use crate::column::{Column};
use crate::schema::Schema;

pub type PageId = usize;
pub type RecordId = usize;

pub type SchemaRef = Arc<Schema>;

pub type ColumnRef = Arc<Column>;