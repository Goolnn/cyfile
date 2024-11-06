// 头部数据
pub const HEADER_DATA: [u8; 15] = [
    0xE8, 0x8B, 0x8D, 0xE7, 0x9C, 0xBC, 0xE6, 0xB1, 0x89, 0xE5, 0x8C, 0x96, 0xE7, 0xBB, 0x84,
];

// 版本数据
pub const VERSION_0_0: (u8, u8) = (0x00, 0x00);
pub const VERSION_0_1: (u8, u8) = (0x00, 0x01);
pub const VERSION_0_2: (u8, u8) = (0x00, 0x02);

// 最新版本
pub const VERSION_LATEST: (u8, u8) = VERSION_0_2;

// 版本列表
pub const VERSIONS: [(u8, u8); 3] = [VERSION_0_0, VERSION_0_1, VERSION_0_2];
