use derive_with::With;
use crate::global::{PageId, PAGE_SIZE};

#[derive(Debug, Clone, With)]
pub struct Page {
    pub page_id: PageId,
    data: [u8; PAGE_SIZE],
    // 被引用次数
    pub pin_count: u32,
    // 是否被写过
    pub is_dirty: bool,
}