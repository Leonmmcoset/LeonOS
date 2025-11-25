mod constant;
mod ata;
mod init;
mod pio;
mod drive;

use os_in_rust_common::port::Port;

/**
 * 使PC扬声器发出beep声音
 * 使用端口0x61控制扬声器
 */
#[inline(never)]
pub fn beep() {
    // 非常简单的实现，只做基本的扬声器控制
    // 避免复杂的操作可能导致的内存问题
    let port_61 = Port::<u8>::new(0x61);
    
    // 只打开然后立即关闭扬声器，避免长时间操作
    let current_value = port_61.read();
    port_61.write(current_value | 0x03); // 打开扬声器
    port_61.write(current_value & 0xfc); // 关闭扬声器
}

pub use init::get_all_partition;
pub use init::ata_init;
pub use init::get_ata_channel;


pub use ata::Partition;
pub use ata::ChannelIrqNoEnum;
pub use ata::ChannelPortBaseEnum;
pub use ata::Disk;


pub use pio::StatusRegister;