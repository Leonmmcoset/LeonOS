use os_in_rust_common::instruction;
use crate::{thread, println};
use crate::thread::{TaskStruct, TaskStatus, wake_thread, remove_from_ready_thread};

/**
 * 查找指定PID的进程
 */
fn find_task_by_pid(pid: usize) -> Option<&'static mut TaskStruct> {
    // 遍历所有线程，查找匹配的PID
    // Use iter() to get a proper iterator over the LinkedList
    for tag in thread::get_all_thread().iter() {
        // Convert LinkedNode to TaskStruct using parse_by_all_tag
        let task = unsafe { &mut *TaskStruct::parse_by_all_tag(unsafe { &mut *tag }) };
        if task.pid.get_data() as usize == pid {
            return Some(task);
        }
    }
    None
}

/**
 * psend命令的效果：通过pid强制关闭指定进程
 */
pub fn psend(pid_str: &str) {
    // 解析PID参数
    let trimmed_pid_str = pid_str.trim();
    let pid_num = match trimmed_pid_str.parse::<usize>() {
        Ok(pid) => {
            if pid == 0 {
                println!("PID cannot be 0");
                return;
            }
            pid
        },
        Err(_) => {
            println!("Invalid PID format: {}", trimmed_pid_str);
            println!("Usage: psend <pid>");
            return;
        }
    };
    
    // 查找对应pid的进程
    // We need to implement this function since it doesn't exist in either module
    let task = find_task_by_pid(pid_num);
    if let Some(task) = task {
        println!("Sending termination signal to process with PID: {}", pid_num);
        // 强制终止进程
        force_terminate_process(task);
        println!("Process with PID: {} terminated successfully", pid_num);
    } else {
        println!("Process with PID: {} not found", pid_num);
        println!("Use 'ps' command to list all processes");
    }
}

/**
 * 强制终止进程
 */
fn force_terminate_process(task: &mut TaskStruct) {
    let old_status = instruction::disable_interrupt();
    
    // 先获取PID信息，避免borrow issues
    let pid = task.pid.get_data();
    
    // 设置进程状态为退出状态（使用具体的退出码1表示被终止）
    task.exit_status = Some(1);
    
    // 将进程状态设置为挂起（TaskHanging），这会触发系统回收该进程
    task.set_status(TaskStatus::TaskHanging);
    
    // 确保从就绪队列移除，防止被调度执行
    remove_from_ready_thread(task);
    
    // 唤醒进程的父进程（如果在等待）
    if let Some(parent) = task.find_parent() {
        if parent.task_status == TaskStatus::TaskWaiting {
            wake_thread(parent);
        }
    }
    
    // 现在安全地获取进程名，因为所有修改都已完成
    let task_name = task.get_name();
    
    println!("Process '{}' (PID: {}) is being terminated", task_name, pid);
    
    // 恢复中断
    instruction::set_interrupt(old_status);
}