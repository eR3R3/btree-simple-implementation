use crate::global::ColumnRef;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    pub columns: Vec<ColumnRef>,
}

impl Schema {
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn empty() -> Self {
        Self { columns: vec![] }
    }
}