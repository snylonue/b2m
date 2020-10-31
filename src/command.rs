use anyhow::Result;
use std::process::Command;
use std::process::Output;
use std::string::FromUtf8Error;

#[inline]
pub fn parse_output(output: Output) -> Result<(String, String), FromUtf8Error> {
    Ok((
        String::from_utf8(output.stdout)?,
        String::from_utf8(output.stderr)?,
    ))
}
pub fn run_command(cmd: &mut Command) -> Result<(String, String)> {
    Ok(parse_output(cmd.output()?)?)
}
