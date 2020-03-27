use assert_cmd::prelude::*;
use std::error::Error;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_extsts() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("foobar").arg("some/nonexisting/path");
    cmd.assert().failure().stderr(predicate::str::contains("failed to open file"));
    Ok(())
}
