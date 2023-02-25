use std;
use std::{panic};
use std::fmt::{Display, Formatter};
use std::io::Write;

const SHELL_NAME: &str = "simpleshell-rs";

#[derive(Debug)]
enum MyErr {
    E1(String),
}


impl Display for MyErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", SHELL_NAME, &self.to_string()))
    }
}

fn main_loop() {
    let mut input = String::new();
    println!("Welcome to {}.", SHELL_NAME);
    loop {
        print!(">");
        std::io::stdout().flush().unwrap();
        // 读取输入（包含\n）
        let size_ret = std::io::stdin().read_line(&mut input);
        if size_ret.is_err() {
            eprintln!("read input error: {}\n", size_ret.unwrap_err());
            continue;
        }
        let input_len = size_ret.unwrap();
        if input_len == 1 { // 仅含\n
            continue;
        }

        // 解析命令（减一是从\n的位置分隔，结果才不含\n）
        let (input_cmd, _) = input.split_at(input_len - 1);
        // println!("input_cmd: {}", input_cmd);

        let mut cmd_slice: Vec<&str> = input_cmd.splitn(2, " ").collect();

        let cmd = cmd_slice[0];
        let mut args = String::new();
        if cmd_slice.len() == 2 {
            args = String::from(cmd_slice[1]);
        }
        // println!("input cmd: {} {}", cmd_slice[0], cmd_slice[1]);

        // 执行命令
        match panic::catch_unwind(|| { exec(cmd, args.split(" ").filter(|&s| s != "").collect()) }) {
            Ok(()) => {}
            Err(_) => eprintln!("{}: command not found: {:?}\n", SHELL_NAME, input_cmd)
        }

        input.clear();
    }
}


fn exec(cmd: &str, args: Vec<&str>) {
    match cmd {
        "exit" => { std::process::exit(0) }
        _ => {
            let mut cmd = std::process::Command::new(cmd);
            if args.len() > 0 {
                // println!("{:?},", args);
                cmd.args(args);
            }
            let ret = cmd.output();
            match ret {
                Ok(output) => println!("{}", String::from_utf8_lossy(&output.stdout)),
                Err(e) => {
                    eprintln!("{}: exec error: {}\n", SHELL_NAME, e)
                }
            }
        }
    }
}

fn test_cmd() {
    exec("pss", vec![]);
}

fn main() {
    main_loop();
    // test_cmd()
}
