// SPDX-License-Identifier: MIT

use crate::Record;

#[derive(Clone, Debug)]
#[repr(C, align(4))]
pub struct SmmstoreV2 {
    pub record: Record,
    pub num_blocks: u32,
    pub block_size: u32,
    pub mmap_addr: u32,
    pub com_buffer: u32,
    pub com_buffer_size: u32,
    pub apm_cmd: u8,
    pub unused: [u8; 3],
}
