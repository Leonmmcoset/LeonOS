use super::{cmd_custom, cmd_dir, cmd_echo, cmd_file, cmd_grep, cmd_cat};
use super::{cmd::Cmd, cmd_cd, cmd_ls, cmd_ps};

use crate::{print, println};
use crate::sys_call;

#[inline(never)]
pub fn execute_cmd(cwd: &str, cmd: Cmd, param: Option<&str>, buf: &mut [u8]) {
    // 清空缓冲区
    unsafe { buf.as_mut_ptr().write_bytes(0, buf.len()) };
    match cmd { 
        Cmd::Help => {
            println!("Available commands:");
            println!("-------------------------------");
            for (cmd_name, desc) in Cmd::get_all_commands() {
                println!("{:<15} {}", cmd_name, desc);
            }
            println!("-------------------------------");
            println!("Note: Executable files are also supported as commands");
        },
        Cmd::Pwd => {
            print!("{}", cwd);
        },
        Cmd::Ps => {
            cmd_ps::ps();
        },
        Cmd::Ls => {
            cmd_ls::ls(cwd, param, buf);
        },
        Cmd::Cd => {
            let res = cmd_cd::cd(cwd, param, buf);
            if res.is_some() {
                // println!("change directory to {}", res.unwrap());
            } else {
                println!("cd error {} not exist", param.unwrap());
            }
        },
        // 清屏
        Cmd::Clear => {
            sys_call::clear_screen();
        },
        // 创建目录
        Cmd::Mkdir => {
            cmd_dir::mkdir(cwd, param, buf);
        },
        // 删除目录
        Cmd::Rmdir => {
            cmd_dir::rmdir(cwd, param, buf);
        },
        // 创建普通文件
        Cmd::Touch => {
            cmd_file::create_file(cwd, param, buf);
        },
        Cmd::Rm => {
            cmd_file::remove_file(cwd, param, buf);
        },
        Cmd::Shutdown => {
            println!("Shutting down the system...");
            sys_call::shutdown();
        },
        Cmd::Echo => {
            if let Some(params) = param {
                cmd_echo::execute_echo(params);
            } else {
                print!("\n");
            }
        },
        Cmd::Grep => {
            if let Some(params) = param {
                cmd_grep::execute_grep(params);
            } else {
                println!("please input string need to grep");
            }
        },
        Cmd::Cat => {
            if let Some(params) = param {
                cmd_cat::execute_cat(cwd, params, buf);
            } else {
                println!("please input file path");
            }
        },
        Cmd::Custom(cmd) => {
            cmd_custom::custom_cmd(cwd, cmd, param, buf);
        },
    };
}