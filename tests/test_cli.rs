use std::process::Command;
#[test]
fn unreadable_input_fails_instead_of_becoming_empty() {
    for command in ["fix", "badge"] {
        let output = Command::new(env!("CARGO_BIN_EXE_skillcheck"))
            .args([command, "tests"]).output().unwrap();
        assert!(!output.status.success(), "{command} accepted a directory");
    }
}
