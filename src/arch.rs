pub mod x86_64 {
    pub const R15: usize = 0;
    pub const R14: usize = 8;
    pub const R13: usize = 16;
    pub const R12: usize = 24;
    pub const RBP: usize = 32;
    pub const RBX: usize = 40;
    pub const R11: usize = 48;
    pub const R10: usize = 56;
    pub const R9: usize = 64;
    pub const R8: usize = 72;
    pub const RAX: usize = 80;
    pub const RCX: usize = 88;
    pub const RDX: usize = 96;
    pub const RSI: usize = 104;
    pub const RDI: usize = 112;
    pub const ORIG_RAX: usize = 120;
    pub const RIP: usize = 128;
    pub const CS: usize = 136;
    pub const EFLAGS: usize = 144;
    pub const RSP: usize = 152;
    pub const SS: usize = 160;
    pub const FS_BASE: usize = 168;
    pub const GS_BASE: usize = 176;
    pub const DS: usize = 184;
    pub const ES: usize = 192;
    pub const FS: usize = 200;
    pub const GS: usize = 208;

    pub const PAGE_SIZE: usize = 4096;
}
