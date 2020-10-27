use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spideog")?;

    cmd.arg("convert-phylo").arg("test/file/doesnt/exist");
    cmd.assert().failure().stderr(predicate::str::contains(
        "IO error: `The system cannot find the path specified. (os error 3)`",
    ));

    Ok(())
}
