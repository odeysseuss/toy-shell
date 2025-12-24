use crate::{
    commands::{
        builtins::{handle_builtins, is_builtin},
        cmds::Cmd,
        utils::{append_err, append_out, print_err, print_out, write_err, write_out},
    },
    parser::parser::Parser,
};
use libc::{STDIN_FILENO, STDOUT_FILENO, close, dup2, fork, pipe, waitpid};
use std::{
    os::{raw::c_int, unix::process::CommandExt},
    process::{Command, exit},
    ptr::null_mut,
};

pub fn handle_pipe(cmd: &mut Cmd, parser: Parser) {
    unsafe {
        let mut pipes: Vec<[c_int; 2]> = Vec::new();
        for _ in 0..parser.pipe.commands.len() - 1 {
            let mut fds: [c_int; 2] = [0; 2];
            if pipe(fds.as_mut_ptr()) == -1 {
                eprintln!("Pipe failed");
                return;
            }
            pipes.push(fds);
        }

        let mut children: Vec<i32> = Vec::new();
        let last_cmd_index = parser.pipe.commands.len() - 1;

        for (i, cmd_toks) in parser.pipe.commands.iter().enumerate() {
            if cmd_toks.is_empty() {
                continue;
            }

            let mut cmd_strings: Vec<String> = Vec::new();
            for token in cmd_toks {
                cmd_strings.push(token.to_string());
            }

            if cmd_strings.is_empty() {
                continue;
            }

            let command = cmd_strings[0].clone();
            let args = if cmd_strings.len() > 1 {
                cmd_strings[1..].to_vec()
            } else {
                Vec::new()
            };

            let is_last_cmd = i == last_cmd_index;

            if is_last_cmd && is_builtin(command.clone()) {
                cmd.name = command.clone();
                cmd.args = args.clone();
                handle_builtins(cmd);
                handle_redir(cmd, parser);
                return;
            }

            let pid = fork();
            if pid < 0 {
                eprintln!("Fork failed");
                for fds in &pipes {
                    close(fds[0]);
                    close(fds[1]);
                }
                return;
            }

            if pid == 0 {
                // Child process
                if i > 0 {
                    dup2(pipes[i - 1][0], STDIN_FILENO);
                }

                if i < parser.pipe.commands.len() - 1 {
                    dup2(pipes[i][1], STDOUT_FILENO);
                }

                // close all pipe file descriptors in child
                for fds in &pipes {
                    close(fds[0]);
                    close(fds[1]);
                }

                if is_last_cmd {
                    let mut child_cmd = Cmd::new();
                    child_cmd.name = command.clone();
                    child_cmd.args = args.clone();
                    handle_redir(&mut child_cmd, parser.clone());
                }

                // execute the command
                if args.is_empty() {
                    let error = Command::new(command).exec();
                    eprintln!("Failed to execute command: {:?}", error);
                } else {
                    let mut exec_cmd = Command::new(command);
                    exec_cmd.args(&args);
                    let error = exec_cmd.exec();
                    eprintln!("Failed to execute command: {:?}", error);
                }
                exit(1);
            } else {
                // Parent process
                children.push(pid);

                // close pipe ends
                if i > 0 {
                    close(pipes[i - 1][0]);
                }
                if i < pipes.len() {
                    close(pipes[i][1]);
                }
            }
        }

        // close all remaining pipe file descriptors in parent
        for fds in &pipes {
            close(fds[0]);
            close(fds[1]);
        }

        // wait for all child processes to complete
        for child_pid in &children {
            waitpid(*child_pid, null_mut(), 0);
        }
    }
}

pub fn handle_redir(cmd: &mut Cmd, parser: Parser) {
    if let Some((filename, is_append)) = parser.redir.combined_file {
        if is_append {
            append_err(cmd, filename.clone());
            append_out(cmd, filename);
        } else {
            write_out(cmd, filename.clone());
            write_err(cmd, filename);
        }
    } else {
        if let Some((filename, is_append)) = parser.redir.stdout_file {
            if is_append {
                append_out(cmd, filename);
            } else {
                write_out(cmd, filename);
            }
        } else {
            print_out(cmd);
        }

        if let Some((filename, is_append)) = parser.redir.stderr_file {
            if is_append {
                append_err(cmd, filename);
            } else {
                write_err(cmd, filename);
            }
        } else {
            print_err(cmd);
        }
    }
}
