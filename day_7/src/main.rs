use std::process::{Command, Stdio};

fn main() {

    let path = r#"..\day_7_intcode\target\release\day_7_intcode.exe"#;
    let cmd = Command::new(path)
        .args(&["0", "1"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let output = cmd
    .wait_with_output()
    .expect("failed to wait on child");

    for code in output.stdout {
        print!("{}", code as char);
    }
}
