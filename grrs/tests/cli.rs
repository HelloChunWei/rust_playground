use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use assert_fs::prelude::*;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("foo").arg("tests/file_doesnt_exist.txt");
    cmd.assert().failure().stderr(predicate::str::contains("could not read file"));
    Ok(())
}

#[test] 
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = assert_fs::NamedTempFile::new("file.txt")?;
    temp_file.write_str("lorem ipsum\ndolor sit amet")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("lorem").arg(temp_file.path());
    cmd.assert().success().stdout(predicate::str::contains("lorem ipsum\n"));
    Ok(())
}