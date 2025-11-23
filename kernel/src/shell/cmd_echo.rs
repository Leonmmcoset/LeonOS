// echo 命令实现

use crate::filesystem::{FileDescriptor, StdFileDescriptor};
use crate::print;

/// 执行echo命令，输出参数内容
pub fn execute_echo(args: &str) {
    if args.trim().is_empty() {
        print!("\n");
        return;
    }
    
    // 写入标准输出
    crate::sys_call::write(
        FileDescriptor::new(StdFileDescriptor::StdOutputNo as usize), 
        args.trim().as_bytes()
    );
    print!("\n");
}