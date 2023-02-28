use std;
use std::{env, thread};
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

extern crate termion;

const SHELL_NAME: &str = "simpleshell-rs";


// #[derive(Debug)]
// enum MyErr {
//     E1(String),
// }
//

// impl Display for MyErr {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("{}: {}", SHELL_NAME, &self.to_string()))
//     }
// }

fn get_dirname_of_path(path: &PathBuf) -> &str {
    path.components().last().unwrap().as_os_str().to_str().unwrap()
}

fn main_loop() {
    let mut input = String::new();
    println!("Welcome to {}.", SHELL_NAME);

    let mut current_workdir = env::current_dir().unwrap();

    thread::spawn(|| auto_completion());

    loop {
        print!("{} >", get_dirname_of_path(&current_workdir));
        stdout().flush().unwrap();
        // 读取输入（包含\n）
        let ret = stdin().read_line(&mut input);

        if ret.is_err() {
            eprintln!("read input error: {}\n", ret.unwrap_err());
            continue;
        }
        if input.trim().is_empty() {
            continue;
        }

        let cmd_slice = input.split(";").filter(|&s| s != "");

        // 执行每条命令
        for single_cmd in cmd_slice {
            let mut parts = single_cmd.split_whitespace();
            let cmd = parts.next().unwrap();
            let args = parts;

            exec(cmd, args, &mut current_workdir);
            // 执行命令
            // match panic::catch_unwind(|| { exec(cmd, args, &mut current_workdir) }) {
            //     Ok(()) => {}
            //     Err(_) => eprintln!("{}: exec panic for: {:?}\n", SHELL_NAME, single_cmd)
            // }
        }
        input.clear();
    }
}


fn exec<I: Iterator>(cmd: &str, args: I, workdir: &mut PathBuf) where <I as Iterator>::Item: AsRef<OsStr> {
    match cmd {
        "exit" => { std::process::exit(0) }
        "cd" => {
            for path in args {
                match env::set_current_dir(&PathBuf::from(path.as_ref())) {
                    Err(e) => eprintln!("cd: {}: {}", e, path.as_ref().to_str().unwrap()),
                    Ok(()) => { *workdir = env::current_dir().unwrap() }
                }
                break;
            }
        }
        _ => {
            // 先检查命令是否存在（主要要把输出丢弃，否则会定向到console，影响效果）
            if Command::new(cmd).stdout(Stdio::null()).stderr(Stdio::null()).status().is_err() {
                eprintln!("{}: command not found: {}", SHELL_NAME, cmd);
                return;
            }

            let mut child = Command::new(cmd)
                .args(args)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .unwrap();
            let ret = child.wait();
            if ret.is_err() {
                eprintln!("{}: exec error: {}\n", SHELL_NAME, ret.unwrap_err())
            }
        }
    }
}

fn auto_completion() {}

fn test_cmd() {
    exec("pss", "".split_whitespace(), &mut PathBuf::new());
}

fn main() {
    main_loop();
    // test_cmd()
}
