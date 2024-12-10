// SPDX-License-Identifier: MIT

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum RecordKind {
    Unused = 0x0,
    Memory = 0x1,
    Hwrpb = 0x2,
    Mainboard = 0x3,
    Version = 0x4,
    ExtraVersion = 0x5,
    Build = 0x6,
    CompileTime = 0x7,
    CompileBy = 0x8,
    CompileHost = 0x9,
    CompileDomain = 0xa,
    Compiler = 0xb,
    Linker = 0xc,
    Assembler = 0xd,
    Serial = 0xf,
    Console = 0x10,
    Forward = 0x11,
    Framebuffer = 0x12,
    Gpio = 0x13,
    Timestamps = 0x16,
    CbMemConsole = 0x17,
    MrcCache = 0x18,
    Vbnv = 0x19,
    #[deprecated]
    VbootHandoff = 0x20,
    X86RomMtrr = 0x21,
    Dma = 0x22,
    RamOops = 0x23,
    AcpiGnvs = 0x24,
    #[deprecated = "use BoardConfig"]
    BoardId = 0x25,
    VersionTimestamp = 0x26,
    WifiCalibration = 0x27,
    #[deprecated = "use BoardConfig"]
    RamCode = 0x28,
    SpiFlash = 0x29,
    SerialNo = 0x2a,
    Mtc = 0x2b,
    Vpd = 0x2c,
    #[deprecated = "use BoardConfig"]
    SkuId = 0x2d,
    BootMediaParams = 0x30,
    CbMemEntry = 0x31,
    TscInfo = 0x32,
    MacAddrs = 0x33,
    VbootWorkbuf = 0x34,
    MmcInfo = 0x35,
    TpmCbLog = 0x36,
    Fmap = 0x37,
    PlatformBlobVersion = 0x38,
    SmmstoreV2 = 0x39,
    TpmPpiHandoff = 0x3a,
    BoardConfig = 0x40,
    AcpiCnvs = 0x41,
    TypeCInfo = 0x42,
    AcpiRsdp = 0x43,
    Pcie = 0x44,
    // CMOS-related
    CmosOptionTable = 0xc8,
    Option = 0xc9,
    OptionEnum = 0xca,
    OptionDefaults = 0xcb,
    OptionChecksum = 0xcc,
}

#[derive(Copy, Clone, Debug)]
#[repr(C, align(4))]
pub struct Record {
    pub kind: RecordKind,
    pub size: u32,
}
