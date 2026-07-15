use std::collections::HashMap;

pub const ITALIC: &str = "\x1b[3m";
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";

#[macro_export]
macro_rules! evalpc {
    ($p: expr) => {{
        let rhome = if cfg!(windows) {
            var("USERPROFILE").unwrap()
        } else {
            var("HOME").unwrap()
        };
        let home = Path::new(&rhome);
        let home = home.canonicalize().unwrap_or_else(|_| home.to_path_buf());

        let mut rpath = home.join($p);
        if let Ok(canon) = rpath.canonicalize() {
            rpath = canon;
        } else {
            let mut path = std::path::PathBuf::new();
            for component in rpath.components() {
                match component {
                    std::path::Component::CurDir => {}
                    std::path::Component::ParentDir => {
                        path.pop();
                    }
                    other => path.push(other.as_os_str()),
                }
            }
            rpath = path;
        }

        if rpath == Path::new(&home) {
            println!("Error: Cannot modify home directory");
            return;
        }

        if !rpath.starts_with(&home) {
            println!("Error: target is outside of home path");
            return;
        }
        rpath
    }};
}

#[macro_export]
macro_rules! evalp {
    ($p:expr, $m:expr, $d:expr) => {{
        let path = Path::new($p);
        let rhome = if cfg!(windows) {
            var("USERPROFILE").unwrap()
        } else {
            var("HOME").unwrap()
        };
        let home = Path::new(format!("{rhome}/dummy_shell_home").as_str())
            .canonicalize()
            .unwrap();
        if (path.is_file() || path.is_dir()) {
            let path = path.canonicalize().unwrap();
            if $d && path == home {
                println!("Error: cannot delete home.");
                return;
            } else if !path.starts_with(&home) {
                println!("Error: Out of bounds.");
                #[cfg(debug_assertions)]
                println!("DEBUG: HOME {:?}\nPATH {:?}", home, path);
                return;
            }
            path
        } else {
            println!("Error: No such {}", $m);
            return;
        }
    }};
}

pub enum CommandType {
    Text(String),
    Func(fn(&str, &[&str], &mut HashMap<String, i32>)),
}

pub struct Command {
    pub opt: HashMap<&'static str, CommandType>,
    pub min: u8,
    pub max: Option<u8>,
}
#[macro_export]
macro_rules! exec {
    ($win:expr, $uni:expr) => {{
        let output = if cfg!(windows) { $win } else { $uni };

        let fback = String::from_utf8_lossy(&output.stdout);
        let out: Vec<&str> = fback.lines().collect();

        let strt = out.iter().position(|l| !l.trim().is_empty()).unwrap_or(0);
        let end = out
            .iter()
            .rposition(|l| !l.trim().is_empty())
            .map_or(strt, |i| i + 1);

        println!("\n{}", out[strt..end].join("\n"));
    }};
    ($win:expr, $uni:expr, stat) => {{
        if cfg!(windows) {
            $win
        } else {
            $uni
        };
    }};
}
