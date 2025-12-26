use crate::{
    handler::cmds::Cmd,
    utils::{append_to_file, write_to_file},
};

pub fn print_out(cmd: &mut Cmd) {
    if cmd.stdout.is_empty() {
        print!("{}", "".to_string());
    } else {
        print!("{}", cmd.stdout);
    }
}

pub fn print_err(cmd: &mut Cmd) {
    if cmd.stderr.is_empty() {
        print!("{}", "".to_string());
    } else {
        print!("{}", cmd.stderr);
    }
}

pub fn write_out(cmd: &mut Cmd, filename: String) {
    if cmd.stdout.is_empty() {
        write_to_file("".as_bytes().to_vec(), filename);
    } else {
        write_to_file(cmd.stdout.as_bytes().to_vec(), filename);
    }
}

pub fn write_err(cmd: &mut Cmd, filename: String) {
    if cmd.stderr.is_empty() {
        write_to_file("".as_bytes().to_vec(), filename);
    } else {
        write_to_file(cmd.stderr.as_bytes().to_vec(), filename);
    }
}

pub fn append_out(cmd: &mut Cmd, filename: String) {
    if cmd.stdout.is_empty() {
        append_to_file("".as_bytes().to_vec(), filename);
    } else {
        append_to_file(cmd.stdout.as_bytes().to_vec(), filename);
    }
}

pub fn append_err(cmd: &mut Cmd, filename: String) {
    if cmd.stderr.is_empty() {
        append_to_file("".as_bytes().to_vec(), filename);
    } else {
        append_to_file(cmd.stderr.as_bytes().to_vec(), filename);
    }
}
