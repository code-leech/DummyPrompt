use crate::types::{BOLD, Command, CommandType::Text, ITALIC, RESET};
use std::collections::HashMap;
pub fn helpcmd() -> Command {
    Command {
        opt: HashMap::from([
            (
                "",
                Text(format!(
                    "This is the help program.\n\
                    * {BOLD}help{RESET} {ITALIC}-i{RESET}: Show system manual\n\
                    * {BOLD}help{RESET} {ITALIC}-h{RESET}: Show help usage\n\
                    * {BOLD}help{RESET} {ITALIC}-c{RESET}: Show command help"
                )),
            ),
            (
                "-c",
                Text(format!(
                    "This command shows command usage. For more information, use [{BOLD}help{RESET} {ITALIC}-c <command>{RESET}].\n\
                    * {BOLD}echo{RESET}: Echo input back to the user\n\
                    * {BOLD}help{RESET}: Show help information\n\
                    * {BOLD}ls{RESET}: List files in current directory\n\
                    * {BOLD}cd{RESET}: Enter a directory\n\
                    * {BOLD}del{RESET}: Delete a file/directory\n\
                    * {BOLD}edit{RESET}: Edit a file\n\
                    * {BOLD}setvar{RESET}: Set a variable\n\
                    * {BOLD}getvar{RESET}: Get a variable's value\n\
                    * {BOLD}clearvar{RESET}: Clear a variable\n\
                    * {BOLD}whoami{RESET}: Show username\n\
                    * {BOLD}clear{RESET}: Clear terminal screen\n\
                    * {BOLD}exit{RESET}: Exit dummyshell"
                )),
            ),
            (
                "-h",
                Text(format!(
                    "When in [...], commands are written in bold, whilst arguments are written in italics. An argument\n\
                    is an additional detail that you are telling the computer to use. Some commands (e.g. [{BOLD}echo{RESET}])\n\
                    require arguments, otherwise they can't do anything. Other commands (e.g. [{BOLD}whoami{RESET}])\n\
                    don't accept any arguments, since they aren't asking for any since they only output one type of answer.\n\n\
                    Sometimes, you will see a placeholder, which can either use {ITALIC}<...>{RESET} or {ITALIC}(...){RESET} with a single argument if optional.\n\
                    If a {ITALIC}(...){RESET} contains more than one argument, these arguments should be either used together or not used at all.\n\n\
                    A table of available options is often provided. Tables are denoted using {ITALIC}*{RESET}."
                )),
            ),
            (
                "-c clear",
                Text(format!(
                    "This command will clear the terminal screen.\nUsage: [{BOLD}clear{RESET}]"
                )),
            ),
            (
                "-c exit",
                Text(format!(
                    "This command will exit dummyshell.\nUsage: [{BOLD}exit{RESET}]"
                )),
            ),
            (
                "-c echo",
                Text(format!(
                    "This command will echo the input back to the user.\nUsage: [{BOLD}echo{RESET} {ITALIC}<text>{RESET}]"
                )),
            ),
            (
                "-c help",
                Text(format!(
                    "This command will show the help menu.\nUsage: [{BOLD}help{RESET} {ITALIC}<options>{RESET}]\n\
                    * {BOLD}-i{RESET}: Show system manual\n\
                    * {BOLD}-h{RESET}: Show manual usage\n\
                    * {BOLD}-c{RESET}: Show command usage"
                )),
            ),
            (
                "-c setvar",
                Text(format!(
                    "This command will set a variable.\nUsage: [{BOLD}setvar{RESET} {ITALIC}<name>{RESET} {ITALIC}<number>{RESET}]"
                )),
            ),
            (
                "-c getvar",
                Text(format!(
                    "This command will get a variable's value.\nUsage: [{BOLD}getvar{RESET} ({ITALIC}(name){RESET})]"
                )),
            ),
            (
                "-c clearvar",
                Text(format!(
                    "This command will clear a variable.\nUsage: [{BOLD}clearvar{RESET} {ITALIC}(name){RESET}]"
                )),
            ),
            (
                "-c whoami",
                Text(format!(
                    "This command will show the current user's username.\nUsage: [{BOLD}whoami{RESET}]"
                )),
            ),
            (
                "-i",
                Text(format!(
                    "This is a mini {ITALIC}\"shell\"{RESET}. This program's made to teach you how to use one!\n\
                    A shell is an app that allows you to run anything on the computer, except \n\
                    that you have to type in what you want to do. Most developer programs ({ITALIC}ie{RESET} Python)\n\
                    are shell-based, since the shell can allow more complex tasks than a simple mouse\n\
                    and button.\n\nIt is also {ITALIC}much{RESET} faster!\n\n\
                    It's a good idea to learn how to understand the help program's instructions.\nYou should use \
                    [{BOLD}help{RESET} {ITALIC}-h{RESET}] for an explanation. \n\nTry to use this program like a normal computer.\n\
                    A good way to start is to use [{BOLD}ls{RESET}] to display all files inside your home folder. Don't worry, you\n\
                    can't damage any files. To continue learning, do basic daily tasks like viewing, editing, and deleting files\n\
                    Use the [{BOLD}help{RESET} {ITALIC}-c{RESET}] to discover more commands.\n\n\
                    Remember, messing around in a safe environment is the best way to learn!"
                )),
            ),
        ]),
        min: 0,
        max: None,
    }
}
