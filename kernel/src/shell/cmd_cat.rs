// cat 命令实现

use crate::filesystem::{FileDescriptor, StdFileDescriptor};
use crate::print;
use crate::println;
use crate::sys_call;
use super::shell_util;

/// 执行cat命令，显示文件内容
pub fn execute_cat(cwd: &str, file_path: &str, buf: &mut [u8]) {
    if file_path.trim().is_empty() {
        println!("please input file path");
        return;
    }
    
    // 获取绝对路径
    let abs_path_result = shell_util::get_abs_path(cwd, file_path.trim(), buf);
    if abs_path_result.is_err() {
        println!("failed to parse file path: {:?}", abs_path_result.unwrap_err());
        return;
    }
    
    let abs_path = abs_path_result.unwrap();
    
    // 打开文件
    let file = sys_call::File::open(abs_path);
    if file.is_err() {
        println!("failed to cat, file:{}, error: {:?}", abs_path, file.unwrap_err());
        return;
    }
    
    let file = file.unwrap();
    let mut read_buffer = [0u8; 256]; // 使用固定大小的缓冲区
    
    loop {
        // 清空缓冲区
        unsafe { read_buffer.as_mut_ptr().write_bytes(0, read_buffer.len()) };
        
        // 读取文件数据
        let read_bytes = file.read(&mut read_buffer);
        if read_bytes == 0 {
            break; // 文件读取完毕
        }
        
        // 尝试将读取的字节转换为字符串并输出
        if let Ok(content) = core::str::from_utf8(&read_buffer[..read_bytes]) {
            print!("{}", content);
        } else {
            // 如果不能转换为UTF-8字符串，则直接输出字节
            sys_call::write(
                FileDescriptor::new(StdFileDescriptor::StdOutputNo as usize),
                &read_buffer[..read_bytes]
            );
        }
    }
}