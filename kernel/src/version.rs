/**
 * LeonOS 版本信息
 */

/**
 * 主版本号
 */
pub const VERSION_MAJOR: u32 = 1;

/**
 * 次版本号
 */
pub const VERSION_MINOR: u32 = 0;

/**
 * 补丁版本号
 */
pub const VERSION_PATCH: u32 = 0;

/**
 * 构建版本号
 */
pub const VERSION_BUILD: u32 = 1;

/**
 * 完整版本字符串
 */
pub const VERSION_STRING: &str = const_format::formatcp!("{}.{}.{}.{}", VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH, VERSION_BUILD);

/**
 * 版本名称
 */
pub const VERSION_NAME: &str = "LeonOS Inside Developer (Alpha) Test";
