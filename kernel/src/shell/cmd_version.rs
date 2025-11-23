/**
 * version命令实现 - 显示系统版本信息
 */

use crate::println;
use crate::version::{VERSION_STRING, VERSION_NAME};

/**
 * 执行version命令，显示系统版本信息
 */
#[inline(never)]
pub fn version() {
    println!("Version: {}", VERSION_STRING);
    println!("Name: {}", VERSION_NAME);
}