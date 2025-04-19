use std::cmp::Ordering;
use crate::column::ScalarValue;
use crate::global::SchemaRef;
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tuple {
    pub schema: SchemaRef,
    pub data: Vec<ScalarValue>,
}

impl Tuple {
    pub fn value(&self, index: usize) -> Result<&ScalarValue> {
        self.data.get(index).ok_or(anyhow!("tuple value function error"))
    }
}

impl PartialOrd for Tuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let column_count = self.schema.column_count();
        for idx in 0..column_count {
            // iterate value in each
            let order = self.value(idx).ok()?.partial_cmp(other.value(idx).ok()?)?;
            if order == Ordering::Equal {
                continue;
            } else {
                return Some(order);
            }
        }
        Some(Ordering::Equal)
    }
}