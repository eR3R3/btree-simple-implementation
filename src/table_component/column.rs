#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub default: ScalarValue,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum ScalarValue {
    Int(i32),
    // 你可以扩展支持更多类型
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Int32,
}