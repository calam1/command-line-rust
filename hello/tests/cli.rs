// use std::process::Command;
use assert_cmd::Command; // use this instead of stdlib to access cargo bin

#[test]
fn works() {
    assert!(true);
}

#[test]
fn runs() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn runs_hello() {
    // let mut cmd = Command::new("hello"); // this will fail, can't find hello
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let res = cmd.output();
    assert!(res.is_ok());

    cmd.assert().success().stdout("Hello, world!\n");
}
