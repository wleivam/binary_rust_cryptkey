use std::process::{Command, Output, Stdio};

pub fn output(bin: &str, args: Vec<&str>) -> Output {
    return Command::new(bin).args(args).output().expect("");
}

pub fn pipe(bin: &str, bin_args: Vec<&str>, pipe: &str, pipe_args: Vec<&str>) -> Output {
    return Command::new(pipe)
        .args(pipe_args)
        .stdin(Stdio::from(
            Command::new(bin)
                .args(bin_args)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
                .stdout
                .unwrap(),
        ))
        .output()
        .expect("");
}
