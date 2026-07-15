use crate::{
    evalp, evalpc, exec,
    types::{Command, CommandType::Func, ITALIC, RESET},
};
use crossterm::{
    cursor, execute,
    terminal::{self},
};
use std::env::{current_dir, set_current_dir, var};
use std::fs::{remove_dir_all, remove_file};
use std::io::stdout;
use std::path::Path;
use std::process::{Command as Process, Stdio, exit};
use std::{collections::HashMap, fs::create_dir_all};

pub fn setup() -> HashMap<&'static str, Command> {
    HashMap::from([
        ("help", crate::help::helpcmd()),
        (
            "ls",
            Command {
                opt: HashMap::from([
                    (
                        "",
                        Func(|_, _, _| {
                            exec! {
                                Process::new("cmd").args(["/C", "dir"]).output().unwrap(),
                                Process::new("ls").output().unwrap()

                            }
                        }),
                    ),
                    (
                        "*",
                        Func(|_, argvec, _| {
                            let path = evalp!(argvec[0], "folder", false);
                            exec! {
                                Process::new("cmd").args(["/C", "dir", path.to_str().unwrap()]).output().unwrap(),
                                Process::new("ls").arg(path.to_str().unwrap()).output().unwrap()

                            }
                        }),
                    ),
                ]),
                min: 0,
                max: Some(1),
            },
        ),
        #[cfg(debug_assertions)]
        (
            "dummy",
            Command {
                opt: HashMap::from([(
                    "works*",
                    Func(|argstr, argvec, _| println!("Argstr {argstr}, Argvec {argvec:?}")),
                )]),
                min: 1,
                max: None,
            },
        ),
        (
            "mkdir",
            Command {
                opt: HashMap::from([(
                    "*",
                    Func(|_, argvec, _| {
                        create_dir_all(evalpc!(argvec[0])).unwrap();
                    }),
                )]),
                min: 1,
                max: Some(1),
            },
        ),
        (
            "edit",
            Command {
                opt: HashMap::from([(
                    "*",
                    Func(|_, argvec, _| {
                        let path = evalpc!(argvec[0]);

                        exec!(
                            Process::new("edit")
                                .arg(path.to_str().unwrap())
                                .stdin(Stdio::inherit())
                                .stdout(Stdio::inherit())
                                .stderr(Stdio::inherit())
                                .status()
                                .unwrap(),
                            Process::new("vim")
                                .arg(path.to_str().unwrap())
                                .stdin(Stdio::inherit())
                                .stdout(Stdio::inherit())
                                .stderr(Stdio::inherit())
                                .status()
                                .unwrap(),
                            stat
                        );
                    }),
                )]),
                min: 1,
                max: Some(1),
            },
        ),
        (
            "cd",
            Command {
                opt: HashMap::from([(
                    "*",
                    Func(|_, argvec, _| {
                        set_current_dir(
                            current_dir()
                                .unwrap()
                                .join(evalp!(argvec[0], "folder", false)),
                        )
                        .unwrap();
                    }),
                )]),
                min: 1,
                max: Some(1),
            },
        ),
        (
            "clearvar",
            Command {
                opt: HashMap::from([
                    (
                        "",
                        Func(|_, _, data_table| {
                            data_table.clear();
                            println!("DEBUG: FULL CLEAR!");
                        }),
                    ),
                    (
                        "*",
                        Func(|_, argvec, data_table| {
                            if data_table.contains_key(argvec[0]) {
                                data_table.remove(argvec[0]);
                            } else {
                                println!("Variable not found: {}", argvec[0]);
                            }
                        }),
                    ),
                ]),
                min: 0,
                max: Some(1),
            },
        ),
        (
            "clear",
            Command {
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
            },
        ),
        (
            "echo",
            Command {
                opt: HashMap::from([("", Func(|argstr: &str, _, _| println!("\n{argstr}")))]),
                min: 0,
                max: None,
            },
        ),
        (
            "setvar",
            Command {
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
            },
        ),
        (
            "del",
            Command {
                opt: HashMap::from([(
                    "*",
                    Func(|_, argvec, _| {
                        let path = evalp!(argvec[0], "file/folder", true);
                        if path.is_file() {
                            remove_file(path).unwrap();
                        } else if path.is_dir() {
                            remove_dir_all(path).unwrap();
                        }
                    }),
                )]),
                min: 1,
                max: Some(1),
            },
        ),
        (
            "getvar",
            Command {
                opt: HashMap::from([(
                    "",
                    Func(|_, argvec, data_table| {
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
                    }),
                )]),
                min: 0,
                max: Some(1),
            },
        ),
        (
            "whoami",
            Command {
                opt: HashMap::from([(
                    "",
                    Func(|_, _, _| {
                        if let Some(username) = var("USERNAME").ok().or_else(|| var("USER").ok()) {
                            println!("You are {ITALIC}{username}{RESET}");
                        } else {
                            println!("Could not determine username.");
                        }
                    }),
                )]),
                min: 0,
                max: Some(0),
            },
        ),
        (
            "exit",
            Command {
                opt: HashMap::from([(
                    "",
                    Func(|_, _, _| {
                        exit(0);
                    }),
                )]),
                min: 0,
                max: Some(0),
            },
        ),
    ])
}
