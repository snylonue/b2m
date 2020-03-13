use failure::Error;
use std::process;

type Res<T> = Result<T, Error>;

#[inline]
pub fn parse_output(output: process::Output) -> Res<(String, String)> {
    Ok((String::from_utf8(output.stdout)?, String::from_utf8(output.stderr)?))
}
pub fn run_command(cmd: &mut process::Command) -> Res<(String, String)> {
    parse_output(cmd.output()?)
}