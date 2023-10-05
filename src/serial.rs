// SPDX-License-Identifier: MIT

use super::Record;

#[derive(Copy, Clone, Debug)]
#[repr(C, align(4))]
pub struct Serial {
    pub record: Record,
    pub kind: u32,
    pub baseaddr: u32,
    pub baud: u32,
    pub regwidth: u32,
    pub input_hertz: u32,
}
