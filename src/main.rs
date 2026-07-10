use crossterm::{
    cursor, execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::HashMap;
use std::io::stdin;
use std::io::{Write, stdout};
use std::process::exit;

const ITALIC: &str = "\x1b[3m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

enum CommandType {
    Text(String),
    Func(fn(&str, &[&str], &mut HashMap<String, i32>)),
}

struct Command {
    opt: HashMap<&'static str, CommandType>,
    min: usize,
    max: Option<usize>,
    rawstr: bool,
}

use CommandType::{Func, Text};
fn main() {
    let mut inp = String::new();
    let mut argvec: Vec<&str>;
    let mut argstr: String;
    let mut fcmd: &str;

    execute!(stdout(), EnterAlternateScreen, cursor::MoveTo(0, 0)).unwrap();

    let help_cmd = Command {
        opt: HashMap::from([
            (
                "",
                Text(format!(
                    "This is the help program.\n- {BOLD}help{RESET} {ITALIC}-c{RESET}: Show command help\n\
                    - {BOLD}help{RESET} {ITALIC}-o{RESET}: Show system usage"
                )),
            ),
            (
                "-c",
                Text(format!(
                    "Commands: echo, help, setvar, getvar, clearvar\nUse [{BOLD}help{RESET} {ITALIC}-c{RESET} {ITALIC}*{RESET}] to find command usage.\n\
                    Here is a list of commands:\n\
                    - {BOLD}echo{RESET}: Echo input back to the user\n\
                    - {BOLD}help{RESET}: Show help information\n\
                    - {BOLD}setvar{RESET}: Set a variable\n\
                    - {BOLD}getvar{RESET}: Get a variable's value\n\
                    - {BOLD}clearvar{RESET}: Clear a variable\n\
                    - {BOLD}whoami{RESET}: Show username"
                )),
            ),
            (
                "-c echo",
                Text(format!(
                    "This command will echo the input back to the user.\nUsage: {BOLD}echo{RESET} {ITALIC}<text>{RESET}"
                )),
            ),
            (
                "-c help",
                Text(format!(
                    "This command will show the help menu.\nUsage: {BOLD}help{RESET} [options]"
                )),
            ),
            (
                "-c setvar",
                Text(format!(
                    "This command will set a variable.\nUsage: {BOLD}setvar{RESET} {ITALIC}<name>{RESET} {ITALIC}<value>{RESET}"
                )),
            ),
            (
                "-c getvar",
                Text(format!(
                    "This command will get a variable's value.\nUsage: {BOLD}getvar{RESET} {ITALIC}<name>{RESET}"
                )),
            ),
            (
                "-c clearvar",
                Text(format!(
                    "This command will clear a variable.\nUsage: {BOLD}clearvar{RESET} {ITALIC}<name>{RESET}"
                )),
            ),
            (
                "-c whoami",
                Text(format!(
                    "This command will show the current user's username.\nUsage: {BOLD}whoami{RESET}"
                )),
            ),
            (
                "-o",
                Text(format!(
                    "This is a mock {ITALIC}\"shell\"{RESET}. This is to educate the user on how to use a shell.\n\
                        A shell is like an app that allows you to run anything on the computer, except \n\
                        that you have to type in what you want to do. Most developer programs ({ITALIC}ie{RESET} Python)\n\
                        are shell-based, since the shell can allow more complex tasks than a simple mouse\n\
                        and button.\n\nIt is also {ITALIC}much{RESET} faster!"
                )),
            ),
        ]),
        min: 0,
        max: None,
        rawstr: false,
    };

    let whoami_cmd = Command {
        opt: HashMap::from([(
            "",
            Func(|_, _, _| {
                if let Some(username) = std::env::var("USERNAME")
                    .ok()
                    .or_else(|| std::env::var("USER").ok())
                {
                    println!("You are {ITALIC}{username}{RESET}");
                } else {
                    println!("Could not determine username.");
                }
            }),
        )]),
        min: 0,
        max: Some(0),
        rawstr: false,
    };

    let setvar_cmd = Command {
        opt: HashMap::from([(
            "",
            Func(
                |_, argvec: &[&str], data_table: &mut HashMap<String, i32>| {
                    if let Ok(val) = argvec[1].parse::<i32>() {
                        data_table.insert(argvec[0].to_string(), val);
                    } else {
                        println!("Invalid value: {}", argvec[1]);
                    }
                },
            ),
        )]),
        min: 2,
        max: Some(2),
        rawstr: false,
    };

    let getvar_cmd = Command {
        opt: HashMap::from([(
            "",
            Func(
                |_, argvec: &[&str], data_table: &mut HashMap<String, i32>| {
                    if argvec.is_empty() {
                        if data_table.is_empty() {
                            println!("No variables set.");
                        } else {
                            println!();
                            for (var, val) in data_table.iter() {
                                println!("{var} = {val}");
                            }
                        }
                        return;
                    }
                    if let Some(val) = data_table.get(argvec[0]) {
                        println!("\n{} = {}", argvec[0], val);
                    } else {
                        println!("Variable not found: {}", argvec[0]);
                    }
                },
            ),
        )]),
        min: 0,
        max: Some(1),
        rawstr: false,
    };

    let clear_cmd = Command {
        opt: HashMap::from([(
            "",
            Func(|_, _, _| {
                execute!(
                    stdout(),
                    terminal::Clear(terminal::ClearType::All),
                    terminal::Clear(terminal::ClearType::Purge),
                    cursor::MoveTo(0, 0)
                )
                .unwrap();
            }),
        )]),
        min: 0,
        max: Some(0),
        rawstr: false,
    };

    let echo_cmd = Command {
        opt: HashMap::from([("", Func(|argstr: &str, _, _| println!("\n{argstr}")))]),
        min: 0,
        max: None,
        rawstr: true,
    };

    let exit_cmd = Command {
        opt: HashMap::from([(
            "",
            Func(|_, _, _| {
                execute!(stdout(), LeaveAlternateScreen).unwrap();
                exit(0);
            }),
        )]),
        min: 0,
        max: Some(0),
        rawstr: false,
    };

    let commands = HashMap::from([
        ("help", help_cmd),
        ("clear", clear_cmd),
        ("echo", echo_cmd),
        ("setvar", setvar_cmd),
        ("getvar", getvar_cmd),
        ("whoami", whoami_cmd),
        ("exit", exit_cmd),
    ]);

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

        if let Some(cmd_map) = commands.get(fcmd) {
            if let Some(cmd) = cmd_map
                .opt
                .get(argstr.as_str())
                .or_else(|| cmd_map.opt.get([argvec[0], "*"].concat().as_str()))
                .or_else(|| {
                    if cmd_map.opt.len() == 1 {
                        cmd_map.opt.get("")
                    } else {
                        None
                    }
                })
            {
                if argvec.len() >= cmd_map.min
                    && (!argstr.contains('*') || cmd_map.rawstr)
                    && cmd_map.max.is_none_or(|max| argvec.len() <= max)
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
