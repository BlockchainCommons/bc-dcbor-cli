use anyhow::{Result, bail};

pub fn run_cli_raw_stdin(args: &[&str], stdin: &str) -> Result<String> {
    let output = assert_cmd::cargo::cargo_bin_cmd!("dcbor")
        .args(args)
        .write_stdin(stdin)
        .assert();

    if output.get_output().status.success() {
        Ok(String::from_utf8(output.get_output().stdout.to_vec()).unwrap())
    } else {
        bail!(
            "Command failed: {:?}",
            String::from_utf8(output.get_output().stderr.to_vec()).unwrap()
        );
    }
}

pub fn run_cli(args: &[&str]) -> Result<String> {
    run_cli_raw_stdin(args, "").map(|s| s.trim().to_string())
}

pub fn run_cli_expect(args: &[&str], expected: &str) -> Result<()> {
    let output = run_cli(args)?;
    if output != expected.trim() {
        bail!(
            "\n\n=== Expected ===\n{}\n\n=== Got ===\n{}",
            expected,
            output
        );
    }
    assert_eq!(expected.trim(), output);
    Ok(())
}
