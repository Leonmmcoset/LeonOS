
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cmd<'a> {
    Pwd,
    Cd,
    Ps, 
    Ls,
    Clear,
    Mkdir,
    Rmdir,
    Touch,
    Rm,
    Shutdown,
    Help,
    Echo,
    Grep,
    Cat,
    Version,
    Date,
    Psend,
    Hello,
    Custom(&'a str)
}
impl <'a> Cmd<'a> {
    // pub fn get_name(&self) -> &str {
    //     match self {
    //         Cmd::Cwd => "cwd",
    //         Cmd::Ps => "ps",
    //         Cmd::Ls => "ls",
    //         Cmd::Cd => "cd",
    //     }
    // }
    pub fn get_by_name(name: &'a str) -> Self {
        match name {
            "pwd" => Self::Pwd,
            "cd" => Self::Cd,
            "ps" => Self::Ps,
            "ls" => Self::Ls,
            "clear" => Self::Clear,
            "mkdir" => Self::Mkdir,
            "rmdir" => Self::Rmdir,
            "touch" => Self::Touch,
            "rm" => Self::Rm,
            "shutdown" => Self::Shutdown,
            "help" => Self::Help,
            "echo" => Self::Echo,
            "grep" => Self::Grep,
        "cat" => Self::Cat,
        "version" => Self::Version,
        "date" => Self::Date,
        "psend" => Self::Psend,
        "hello" => Self::Hello,
        _ => Cmd::Custom(name),
        }
    }
    
    /// Get all built-in commands with their descriptions
    pub fn get_all_commands() -> &'static [(&'static str, &'static str)] {
        &[
            ("pwd", "Show current directory"),
            ("cd", "Change directory"),
            ("ps", "List all processes"),
            ("ls", "List directory contents"),
            ("clear", "Clear screen"),
            ("mkdir", "Create new directory"),
            ("rmdir", "Remove directory"),
            ("touch", "Create new file"),
            ("rm", "Remove file"),
            ("shutdown", "Shutdown system"),
            ("help", "Show all available commands"),
            ("echo", "Print arguments to stdout"),
            ("grep", "Filter lines containing a pattern"),
            ("cat", "Display file contents"),
            ("version", "Show system version information"),
            ("date", "Show current date and time"),
            ("psend", "Force terminate process by PID"),
            ("hello", "Print 'hello world' message"),
        ]
    }
}



