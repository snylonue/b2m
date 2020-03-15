use std::process;
use std::string::FromUtf8Error;
use super::Res;

#[inline]
pub fn parse_output(output: process::Output) -> Result<(String, String), FromUtf8Error> {
    Ok((String::from_utf8(output.stdout)?, String::from_utf8(output.stderr)?))
}
pub fn run_command(cmd: &mut process::Command) -> Res<(String, String)> {
    Ok(parse_output(cmd.output()?)?)
}