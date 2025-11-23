// grep 命令实现

use crate::filesystem::{FileDescriptor, StdFileDescriptor};
use crate::println;
use crate::sys_call;

/// 执行grep命令，过滤包含指定字符串的行
pub fn execute_grep(pattern: &str) {
    if pattern.trim().is_empty() {
        println!("please input string need to grep");
        return;
    }
    
    let grep_str = pattern.trim();
    let mut line_buffer = [0u8; 1024];
    let mut line_len = 0;
    
    println!("Input text (press Ctrl+D to finish):");
    
    // 使用标准的read函数读取输入，避免使用可能有问题的read_key
    loop {
        let mut ch = [0u8; 1];
        let bytes_read = sys_call::read(
            FileDescriptor::new(StdFileDescriptor::StdInputNo as usize),
            &mut ch
        );
        
        // 如果读取不到数据，退出循环
        if bytes_read == 0 {
            break;
        }
        
        let key_char = ch[0];
        
        // 处理回车键或换行符
        if key_char == b'\r' || key_char == b'\n' {
            // 回显换行符
            sys_call::write(
                FileDescriptor::new(StdFileDescriptor::StdOutputNo as usize),
                &[key_char]
            );
            
            if line_len > 0 {
                // 将缓冲区转换为字符串
                if let Ok(line) = core::str::from_utf8(&line_buffer[..line_len]) {
                    // 如果行包含搜索字符串，则输出
                    if line.contains(grep_str) {
                        println!("{}", line);
                    }
                }
                line_len = 0; // 重置行缓冲区
            }
            continue;
        }
        
        // 处理Ctrl+C（通常是0x03）
        if key_char == 0x03 {
            println!("^C");
            break;
        }
        
        // 回显字符到屏幕
        sys_call::write(
            FileDescriptor::new(StdFileDescriptor::StdOutputNo as usize),
            &[key_char]
        );
        
        // 将字符添加到行缓冲区
        if line_len < line_buffer.len() - 1 {
            line_buffer[line_len] = key_char;
            line_len += 1;
        }
    }
    
    // 处理最后一行（如果没有换行符结尾）
    if line_len > 0 {
        if let Ok(line) = core::str::from_utf8(&line_buffer[..line_len]) {
            if line.contains(grep_str) {
                println!("{}", line);
            }
        }
    }
    
    println!();
}