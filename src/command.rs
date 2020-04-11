use std::process::Output;
use std::process::Command;
use std::string::FromUtf8Error;
use crate::Res;

#[inline]
pub fn parse_output(output: Output) -> Result<(String, String), FromUtf8Error> {
    Ok((String::from_utf8(output.stdout)?, String::from_utf8(output.stderr)?))
}
pub fn run_command(cmd: &mut Command) -> Res<(String, String)> {
    Ok(parse_output(cmd.output()?)?)
}