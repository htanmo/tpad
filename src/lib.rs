use colored::Colorize;
use directories::BaseDirs;
use indexmap::IndexSet;
use std::{
    collections::HashMap,
    env,
    fs::{self, File, OpenOptions},
    io::{self, prelude::*, BufReader, BufWriter},
    path::{Path, PathBuf},
    process,
};

pub const HELP: &'static str = "Usage: tpad [COMMAND] [ARGUMENTS]
Taskpad or tpad is a command line todo utility for comfort.

Example: tpad list

Available commands:
    - add [TASK/s]
        adds new task(s)
        Example: tpad add \"contribute\"
    - init
        initializes a .tpad file in current directory
        Example: tpad init
    - list
        lists all tasks
        Example: tpad list
    - done [INDEX]
        marks task as done
        Example: tpad done 2 4 (marks second and fourth task as completed)
    - undo [INDEX]
        marks task as undone
        Example: tpad undo 2 4 (marks second and fourth task as incomplete)
    - rm [INDEX]/[done]
        removes task(s)
        Example: tpad rm 2
                 tpad rm done (removes all completed tasks)
    - reset
        deletes all tasks
    - restore
        restore recent backup after reset
";

#[derive(Debug)]
pub struct Task {
    todo: Vec<String>,
    todo_path: String,
    backed_up: bool,
    backup_path: String,
}

impl Task {
    pub fn new() -> Self {
        let dir = match BaseDirs::new() {
            Some(dir) => dir,
            None => {
                eprintln!("{}", "failed to create dir object!".red());
                process::exit(1);
            }
        };

        // path to home directory
        let home_dir_path = dir.home_dir().to_path_buf();

        let path = Path::new("./.tpad");
        let todo_path = if path.exists() {
            path.display().to_string()
        } else {
            match env::var("TASKPAD") {
                Ok(value) => {
                    let mut path = PathBuf::from(value);
                    if path.is_dir() {
                        path.push(".tpad");
                        path.display().to_string()
                    } else {
                        let mut path = home_dir_path.clone();
                        path.push(".tpad");
                        path.display().to_string()
                    }
                }
                Err(_) => {
                    let mut path = dir.home_dir().to_path_buf();
                    path.push(".tpad");
                    path.display().to_string()
                }
            }
        };

        let mut backed_up = false;
        let backup_path = match env::var("TASKPAD_BACKUP") {
            Ok(value) => {
                let mut path = PathBuf::from(value);
                if path.is_dir() {
                    path.push(".tpad.bak");
                    if path.exists() {
                        backed_up = true;
                    }
                    path.display().to_string()
                } else {
                    let mut path = home_dir_path.clone();
                    path.push(".tpad.bak");
                    if path.exists() {
                        backed_up = true;
                    }
                    path.display().to_string()
                }
            }
            Err(_) => {
                let mut path = home_dir_path.clone();
                path.push(".tpad.bak");
                if path.exists() {
                    backed_up = true;
                }
                path.display().to_string()
            }
        };

        let todofile = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&todo_path)
            .expect("failed to open todo file!");

        let mut buf_reader = BufReader::new(todofile);

        let mut contents = String::new();

        buf_reader
            .read_to_string(&mut contents)
            .expect("failed to read to string!");

        let todo: IndexSet<String> =
            contents.lines().map(str::to_string).collect();
        let todo = Vec::from_iter(todo.into_iter());

        Self { todo, todo_path, backed_up, backup_path }
    }

    pub fn init(&self) {
        let path = Path::new("./.tpad");
        if !path.exists() {
            File::create(path).expect("failed to create .tpad file");
        } else {
            eprintln!("{}", ".tpad file already exists!".red());
        }
    }

    pub fn add(&self, args: &[String]) {
        if args.len() == 0 {
            eprintln!("{}", "tpad takes atleast one argument!".red());
            process::exit(1);
        }

        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&self.todo_path)
            .expect("failed to open file!");

        let mut bufwriter = BufWriter::new(file);

        for todo in args {
            bufwriter
                .write_all(format!("[ ] {}\n", todo).as_bytes())
                .expect("failed to write!");
        }
    }

    pub fn list(&self) {
        if self.todo.len() == 0 {
            process::exit(1);
        }
        let stdout = io::stdout();
        let mut bufwriter = BufWriter::new(stdout);

        for (index, todo) in self.todo.iter().enumerate() {
            if todo.len() > 4 {
                let status: String;
                let task: String;
                match &todo[..4] {
                    "[ ] " => {
                        status = "[ ] ".to_string();
                        task = format!("{}", &todo[4..]);
                    }
                    "[*] " => {
                        status = format!("[{}] ", "*".red());
                        task =
                            format!("{}", &todo[4..].strikethrough().dimmed());
                    }
                    _ => unreachable!(),
                }
                let idx = index + 1;
                let line = format!("{}{}. {}\n", status, idx, task);
                bufwriter
                    .write_all(line.as_bytes())
                    .expect("failed to write to stdout");
            }
        }
    }

    pub fn done(&mut self, index: &[String]) {
        if index.is_empty() {
            eprintln!("{}", "tpad done takes atleast one argument!".red());
            process::exit(1);
        }

        if self.todo.len() == 0 {
            process::exit(1);
        }

        let idx: Vec<usize> = index
            .iter()
            .map(|x| match x.trim().parse::<usize>() {
                Ok(v) => v - 1,
                Err(_) => {
                    eprintln!("{} {}", x.red(), "not a valid index!".red());
                    process::exit(1);
                }
            })
            .collect();

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("failed to open the file!");

        let mut bufwriter = BufWriter::new(file);

        for i in idx {
            if i > self.todo.len() - 1 {
                continue;
            }
            let task = &self.todo[i][4..];
            self.todo[i] = format!("[*] {}", task);
        }

        for tasks in &self.todo {
            bufwriter
                .write_all(format!("{}\n", tasks).as_bytes())
                .expect("failed to write to the file");
        }
    }

    pub fn undo(&mut self, index: &[String]) {
        if index.is_empty() {
            eprintln!("{}", "tpad undo takes atleast one argument!".red());
            process::exit(1);
        }

        if self.todo.len() == 0 {
            process::exit(1);
        }

        let idx: Vec<usize> = index
            .iter()
            .map(|x| match x.trim().parse::<usize>() {
                Ok(v) => v - 1,
                Err(_) => {
                    eprintln!("{} {}", x.red(), "not a valid index!".red());
                    process::exit(1);
                }
            })
            .collect();

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("failed to open the file!");

        let mut bufwriter = BufWriter::new(file);

        for i in idx {
            if i > self.todo.len() - 1 {
                continue;
            }
            let task = &self.todo[i][4..];
            self.todo[i] = format!("[ ] {}", task);
        }

        for tasks in &self.todo {
            bufwriter
                .write_all(format!("{}\n", tasks).as_bytes())
                .expect("failed to write to the file");
        }
    }

    pub fn remove(&mut self, index: &[String]) {
        if index.is_empty() {
            eprintln!("{}", "tpad rm takes atleast one arguement!".red());
            process::exit(1);
        }

        if self.todo.len() == 0 {
            process::exit(1);
        }

        let mut map = HashMap::new();

        for i in self.todo.iter().enumerate() {
            map.insert(i.0, i.1);
        }

        let mut idx: Vec<usize> = Vec::new();
        if index[0] == "done" && index.len() == 1 {
            for (i, s) in self.todo.iter().enumerate() {
                if s.as_str().starts_with("[*]") {
                    idx.push(i);
                }
            }
        } else {
            idx = index
                .iter()
                .map(|x| match x.trim().parse::<usize>() {
                    Ok(v) => v - 1,
                    Err(_) => {
                        eprintln!(
                            "{} {}",
                            x.red(),
                            "is not a valid index!".red()
                        );
                        process::exit(1);
                    }
                })
                .collect();
        }

        for i in idx {
            if i > self.todo.len() - 1 {
                continue;
            }
            map.remove(&i);
        }

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("failed to open the file!");

        let mut bufwriter = BufWriter::new(file);
        for todo in map {
            bufwriter
                .write_all(format!("{}\n", todo.1).as_bytes())
                .expect("failed to write to the file");
        }
    }

    pub fn restore(&self) {
        if self.backed_up {
            fs::copy(&self.backup_path, &self.todo_path)
                .expect("failed to backup!");
        } else {
            File::create(&self.backup_path)
                .expect("failed to create the backup file!");
        }
    }

    pub fn reset(&self) {
        if self.todo.len() == 0 {
            process::exit(1);
        }
        fs::copy(&self.todo_path, &self.backup_path)
            .expect("failed to backup!");
        File::create(&self.todo_path).expect("failed to truncate file!");
    }
}
