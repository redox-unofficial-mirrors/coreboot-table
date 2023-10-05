use super::{Cb64, Record};

#[derive(Copy, Clone, Debug)]
#[repr(C, align(4))]
pub struct Forward {
    pub record: Record,
    pub forward: Cb64,
}
