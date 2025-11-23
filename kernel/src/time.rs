use core::sync::atomic::{AtomicU64, Ordering};
use core::mem::size_of;

/**
 * 简单的时间管理模块
 * 由于系统中没有RTC实现，我们使用一个基于PIT定时器的简单时间计数器
 */

// 系统启动时间计数器（以秒为单位）
static SYSTEM_TIME_SECONDS: AtomicU64 = AtomicU64::new(0);

// 系统启动时的默认时间（2024-01-01 00:00:00）
const DEFAULT_YEAR: u16 = 2024;
const DEFAULT_MONTH: u8 = 1;
const DEFAULT_DAY: u8 = 1;
const DEFAULT_HOUR: u8 = 0;
const DEFAULT_MINUTE: u8 = 0;
const DEFAULT_SECOND: u8 = 0;

/**
 * 时间结构体
 */
#[derive(Clone, Copy, Debug)]
pub struct Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Time {
    pub const fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
    
    /**
     * 将时间格式化为字符串
     * 格式：YYYY-MM-DD HH:MM:SS
     */
    pub fn to_string(&self) -> [u8; 20] {
        let mut buffer = [0u8; 20];
        
        // 格式化年份
        buffer[0] = b'0' + ((self.year / 1000) % 10) as u8;
        buffer[1] = b'0' + ((self.year / 100) % 10) as u8;
        buffer[2] = b'0' + ((self.year / 10) % 10) as u8;
        buffer[3] = b'0' + (self.year % 10) as u8;
        buffer[4] = b'-';
        
        // 格式化月份
        buffer[5] = b'0' + (self.month / 10) as u8;
        buffer[6] = b'0' + (self.month % 10) as u8;
        buffer[7] = b'-';
        
        // 格式化日期
        buffer[8] = b'0' + (self.day / 10) as u8;
        buffer[9] = b'0' + (self.day % 10) as u8;
        buffer[10] = b' ';
        
        // 格式化小时
        buffer[11] = b'0' + (self.hour / 10) as u8;
        buffer[12] = b'0' + (self.hour % 10) as u8;
        buffer[13] = b':';
        
        // 格式化分钟
        buffer[14] = b'0' + (self.minute / 10) as u8;
        buffer[15] = b'0' + (self.minute % 10) as u8;
        buffer[16] = b':';
        
        // 格式化秒
        buffer[17] = b'0' + (self.second / 10) as u8;
        buffer[18] = b'0' + (self.second % 10) as u8;
        buffer[19] = b'\0';
        
        buffer
    }
}

/**
 * 从系统时间（秒）计算具体的日期时间
 * 这是一个简化的实现，不处理闰年等复杂情况
 */
fn calculate_datetime(seconds: u64) -> Time {
    let mut time = Time::new(
        DEFAULT_YEAR,
        DEFAULT_MONTH,
        DEFAULT_DAY,
        DEFAULT_HOUR,
        DEFAULT_MINUTE,
        DEFAULT_SECOND,
    );
    
    // 添加经过的秒数
    let mut total_seconds = seconds as u32 + DEFAULT_SECOND as u32;
    
    // 计算分钟
    time.minute = (time.minute as u32 + total_seconds / 60) as u8;
    time.second = (total_seconds % 60) as u8;
    
    // 计算小时
    let mut total_minutes = time.minute as u32;
    time.hour = (time.hour as u32 + total_minutes / 60) as u8;
    time.minute = (total_minutes % 60) as u8;
    
    // 计算天数（简化实现，每月按30天计算）
    let mut total_hours = time.hour as u32;
    let days = total_hours / 24;
    time.hour = (total_hours % 24) as u8;
    
    // 简化的日期计算
    let mut total_days = days + time.day as u32;
    while total_days > 30 {
        total_days -= 30;
        time.month += 1;
        
        if time.month > 12 {
            time.month = 1;
            time.year += 1;
        }
    }
    time.day = total_days as u8;
    
    time
}

/**
 * 获取当前系统时间
 */
pub fn get_current_time() -> Time {
    let seconds = SYSTEM_TIME_SECONDS.load(Ordering::SeqCst);
    calculate_datetime(seconds)
}

/**
 * 更新系统时间（由定时器中断调用）
 * 由于TIMER_INTR_FREQUENCY是100Hz，每100次中断为1秒
 */
pub fn update_system_time() {
    static mut TICK_COUNT: u32 = 0;
    const TICKS_PER_SECOND: u32 = 100; // 与TIMER_INTR_FREQUENCY一致
    
    unsafe {
        TICK_COUNT += 1;
        if TICK_COUNT >= TICKS_PER_SECOND {
            TICK_COUNT = 0;
            SYSTEM_TIME_SECONDS.fetch_add(1, Ordering::SeqCst);
        }
    }
}

/**
 * 初始化时间模块
 */
pub fn time_init() {
    // 重置系统时间
    SYSTEM_TIME_SECONDS.store(0, Ordering::SeqCst);
}