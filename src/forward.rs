use super::record::Record;

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct Forward {
    pub record: Record,
    pub forward: u64,
}
