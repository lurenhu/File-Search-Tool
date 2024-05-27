use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // 分配存储用户输出的指令
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // 执行命令
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error :{e}");
        process::exit(1);
    }
}
