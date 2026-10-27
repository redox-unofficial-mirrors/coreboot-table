// SPDX-License-Identifier: MIT

use super::Cb64;
use crate::Record;

#[derive(Clone, Debug)]
#[repr(C, align(4))]
pub struct SmmstoreV2 {
    pub record: Record,
    /// Number of writable blocks in SMM.
    pub num_blocks: u32,
    /// Size of a block in byte. Default: 64 KiB
    pub block_size: u32,
    /// 32-bit MMIO address of the store for read-only access.
    #[deprecated = "Prefer `mmap_addr` for new software."]
    pub mmap_addr_deprecated: u32,
    /// Physical address of the communication buffer.
    pub com_buffer: u32,
    /// Size of the communication buffer in bytes.
    pub com_buffer_size: u32,
    /// The command byte to write to the APM I/O port.
    pub apm_cmd: u8,
    pub unused: [u8; 3],
    /// 64-bit MMIO address of the store for read only access.
    pub mmap_addr: Cb64,
}
