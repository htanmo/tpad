# Taskpad

Taskpad or tpad is a cli todo utility implemented in rust which makes remembering things hassle free with the comfort of one's terminal.

## Environment Variables

To configure this project, you can set the following environment variables :

`TASKPAD`: Path to directory where you want to store your `.tpad` file

`TASKPAD_BACKUP`: Path to directory where you want to store your `.tpad.bak` file

## Note

- In case of a `.tpad` file being present in the cwd `tpad` will read todos from the same file, even if `TASKPAD` env varible is set.
- Binary for the compiled project can be found under `./target/release/` with the binary name as `tpad` with os specific executable extension.

## How to build

```sh
cargo build --release
```

## Usage/Examples

```text
Usage: tpad [COMMAND] [ARGUMENTS]
Taskpad or tpad is a command line todo utility for productivity.

Example: tpad list

Available commands:
    - add [TASK/s]
        adds new task(s)
        Example: tpad add "contribute"
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
```

## LICENSE
#### [GPL-3.0 license](./COPYING)
