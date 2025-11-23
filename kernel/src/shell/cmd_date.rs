use crate::time::{self, Time};
use os_in_rust_common::printkln;

/**
 * date命令实现
 * 显示当前系统日期和时间
 */

/**
 * 执行date命令
 * 无参数，直接显示当前日期时间
 */
pub fn date() {
    // 获取当前系统时间
    let current_time = time::get_current_time();
    
    // 格式化时间为字符串
    let time_str = current_time.to_string();
    
    // 转换为Rust字符串并输出
    let time_str_slice = core::str::from_utf8(&time_str[..19]).expect("Invalid UTF-8");
    printkln!("{}", time_str_slice);
}