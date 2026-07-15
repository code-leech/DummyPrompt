mod commands;
mod help;
mod setupdir;
mod types;
use crate::setupdir::setup_dir;
use crate::types::{
    BOLD,
    CommandType::{Func, Text},
    RESET,
};
use commands::setup;
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use std::collections::HashMap;
use std::io::stdin;
use std::io::{Write, stdout};

fn main() {
    setup_dir();
    let mut inp = String::new();
    let mut argvec: Vec<&str>;
    let mut sanitvec: Vec<&str>;
    let mut argstr: String;
    let mut sanitstr: String;
    let mut fcmd: &str;

    execute!(
        stdout(),
        Clear(ClearType::All),
        Clear(ClearType::Purge),
        cursor::MoveTo(0, 0)
    )
    .unwrap();
    println!("Welcome! If you are new here, use the [{BOLD}help{RESET}] command!");
    let commands = setup();
    let mut data_table: HashMap<String, i32> = HashMap::new();

    loop {
        print!("{BOLD}> ");
        stdout().flush().unwrap();
        inp.clear();
        stdin().read_line(&mut inp).unwrap();
        argvec = inp.split_whitespace().collect();
        if argvec.is_empty() {
            continue;
        }
        fcmd = argvec.first().unwrap_or(&"");
        argvec.remove(0);

        argstr = argvec.join(" ");

        sanitstr = argstr
            .split_whitespace()
            .map(|w| w.trim_matches('*'))
            .collect::<Vec<&str>>()
            .join(" ");
        sanitvec = sanitstr.split_whitespace().collect();

        if let Some(cmd_map) = commands.get(fcmd) {
            if let Some(cmd) = cmd_map
                .opt
                .get(sanitstr.as_str())
                .or_else(|| {
                    if let Some(first) = sanitvec.first() {
                        #[cfg(debug_assertions)]
                        println!("DEBUG: sanitvec[0] == {first}");
                        cmd_map.opt.get(format!("{first}*").as_str())
                    } else {
                        None
                    }
                })
                .or_else(|| cmd_map.opt.get("*"))
                .or_else(|| {
                    if cmd_map.opt.len() == 1 {
                        cmd_map.opt.get("")
                    } else {
                        None
                    }
                })
            {
                if argvec.len() >= cmd_map.min.into()
                    && cmd_map.max.is_none_or(|max| argvec.len() <= max.into())
                {
                    print!("{RESET}");
                    stdout().flush().unwrap();
                    match cmd {
                        Text(text) => println!("\n{text}"),
                        Func(func) => func(argstr.as_str(), &argvec, &mut data_table),
                    }
                } else {
                    println!("{RESET}Invalid command usage");
                }
            } else {
                println!("{RESET}Invalid command usage: {argstr}");
            }
        } else {
            println!("{RESET}Invalid command: {fcmd}");
        }
    }
}
