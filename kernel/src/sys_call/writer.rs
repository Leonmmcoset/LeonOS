use core::fmt::{self, Write};

use os_in_rust_common::racy_cell::RacyCell;

use crate::filesystem::{FileDescriptor, StdFileDescriptor};

use super::sys_call_proxy;


/**
 * 一个控制台writer，专门写入到控制台
 */
static CONSOLE_WRITER: RacyCell<FileWriter> = RacyCell::new(FileWriter::new(FileDescriptor::new(StdFileDescriptor::StdOutputNo as usize)));

/**
 * 系统调用 print!
 */
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::sys_call::sys_print(format_args!($($arg)*)));
}

/**
 * 系统调用 println!
 */
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/**
 * 带颜色的打印宏
 */
#[macro_export]
macro_rules! print_color {
    ($color:expr, $($arg:tt)*) => {
        {
            let old_attr = unsafe { $crate::vga::WRITER.get_mut().get_current_attr() };
            unsafe { $crate::vga::WRITER.get_mut().set_foreground_color($color) };
            $crate::sys_call::sys_print(format_args!($($arg)*));
            unsafe { $crate::vga::WRITER.get_mut().set_attr(old_attr) };
        }
    };
}

#[macro_export]
macro_rules! println_color {
    ($color:expr, $($arg:tt)*) => {
        $crate::print_color!($color, "{}\n", format_args!($($arg)*));
    };
};

// 常用颜色的快捷宏
#[macro_export]
macro_rules! print_red {
    ($($arg:tt)*) => ($crate::print_color!($crate::vga::Color::Red, $($arg)*));
}

#[macro_export]
macro_rules! println_red {
    ($($arg:tt)*) => ($crate::println_color!($crate::vga::Color::Red, $($arg)*));
}

#[macro_export]
macro_rules! print_green {
    ($($arg:tt)*) => ($crate::print_color!($crate::vga::Color::Green, $($arg)*));
}

#[macro_export]
macro_rules! println_green {
    ($($arg:tt)*) => ($crate::println_color!($crate::vga::Color::Green, $($arg)*));
}

#[macro_export]
macro_rules! print_blue {
    ($($arg:tt)*) => ($crate::print_color!($crate::vga::Color::Blue, $($arg)*));
}

#[macro_export]
macro_rules! println_blue {
    ($($arg:tt)*) => ($crate::println_color!($crate::vga::Color::Blue, $($arg)*));
}

#[macro_export]
macro_rules! print_yellow {
    ($($arg:tt)*) => ($crate::print_color!($crate::vga::Color::Yellow, $($arg)*));
}

#[macro_export]
macro_rules! println_yellow {
    ($($arg:tt)*) => ($crate::println_color!($crate::vga::Color::Yellow, $($arg)*));
};

#[no_mangle]
pub fn sys_print(args: fmt::Arguments) {
    let writer = unsafe { CONSOLE_WRITER.get_mut() };
    writer.write_fmt(args).unwrap();
}

pub struct FileWriter {
    fd: FileDescriptor,
}

impl FileWriter {
    pub const fn new(fd: FileDescriptor) -> Self {
        Self {
            fd,
        }
    }
}


impl Write for FileWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        sys_call_proxy::write(self.fd, s.as_bytes());
        Result::Ok(())
    }
}