use std::ffi::OsString;
use std::path::PathBuf;

use crate::profiles::Policy;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    Version,
    Profiles,
    Check(CheckOptions),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CheckOptions {
    pub profiles: Vec<String>,
    pub path: Option<PathBuf>,
    pub policy: Option<Policy>,
}

pub const HELP: &str = "Usage: tokenkeeper <COMMAND> [OPTIONS]\n\nCommands:\n  check       Inspect configured profiles or an explicit path\n  profiles    List built-in profiles\n\nOptions:\n  -h, --help     Show this help message\n  -V, --version  Show the version\n";

pub fn parse<I>(args: I) -> Result<Command, String>
where
    I: IntoIterator<Item = OsString>,
{
    let mut args = args.into_iter();
    let Some(first) = args.next() else {
        return Ok(Command::Help);
    };

    match first.to_str() {
        Some("-h") | Some("--help") => reject_extra(args, Command::Help),
        Some("-V") | Some("--version") => reject_extra(args, Command::Version),
        Some("profiles") => reject_extra(args, Command::Profiles),
        Some("check") => parse_check(args).map(Command::Check),
        _ => Err(format!("unknown argument `{}`", first.to_string_lossy())),
    }
}

fn reject_extra<I>(mut args: I, command: Command) -> Result<Command, String>
where
    I: Iterator<Item = OsString>,
{
    if let Some(extra) = args.next() {
        return Err(format!("unexpected argument `{}`", extra.to_string_lossy()));
    }
    Ok(command)
}

fn parse_check<I>(args: I) -> Result<CheckOptions, String>
where
    I: Iterator<Item = OsString>,
{
    let mut options = CheckOptions::default();
    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("--profile") => options.profiles.push(
                next_value(&mut args, "--profile")?
                    .to_string_lossy()
                    .into_owned(),
            ),
            Some("--path") => options.path = Some(PathBuf::from(next_value(&mut args, "--path")?)),
            Some("--policy") => {
                options.policy = Some(parse_policy(&next_value(&mut args, "--policy")?)?)
            }
            Some("-h") | Some("--help") => {
                return Err(
                    "check help is not available as a nested command; use `tokenkeeper --help`"
                        .into(),
                )
            }
            _ => return Err(format!("unknown check option `{}`", arg.to_string_lossy())),
        }
    }
    if options.path.is_some() != options.policy.is_some() {
        return Err("--path and --policy must be supplied together".into());
    }
    if options.path.is_none() && options.profiles.is_empty() {
        return Ok(options);
    }
    Ok(options)
}

fn next_value<I>(args: &mut std::iter::Peekable<I>, flag: &str) -> Result<OsString, String>
where
    I: Iterator<Item = OsString>,
{
    args.next()
        .ok_or_else(|| format!("missing value for {flag}"))
}

fn parse_policy(value: &OsString) -> Result<Policy, String> {
    match value.to_str() {
        Some("secret-file") => Ok(Policy::SecretFile),
        Some("credential-config") => Ok(Policy::CredentialConfig),
        Some("private-directory") => Ok(Policy::PrivateDirectory),
        Some("trusted-config") => Ok(Policy::TrustedConfig),
        Some("executable-config") => Ok(Policy::ExecutableConfig),
        _ => Err(format!("unknown policy `{}`", value.to_string_lossy())),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, CheckOptions, Command};
    use crate::profiles::Policy;
    use std::ffi::OsString;

    #[test]
    fn no_arguments_shows_help() {
        assert_eq!(parse(Vec::<OsString>::new()), Ok(Command::Help));
    }

    #[test]
    fn aliases_are_supported() {
        assert_eq!(parse(["-h".into()]), Ok(Command::Help));
        assert_eq!(parse(["-V".into()]), Ok(Command::Version));
    }

    #[test]
    fn extra_arguments_are_rejected() {
        let error = parse(["--help".into(), "extra".into()]).unwrap_err();
        assert!(error.contains("unexpected argument"));
    }

    #[test]
    fn check_accepts_repeatable_profiles() {
        assert_eq!(
            parse([
                "check".into(),
                "--profile".into(),
                "codex".into(),
                "--profile".into(),
                "cursor".into()
            ]),
            Ok(Command::Check(CheckOptions {
                profiles: vec!["codex".into(), "cursor".into()],
                path: None,
                policy: None
            }))
        );
    }

    #[test]
    fn custom_path_requires_policy_and_parses_policy() {
        assert_eq!(
            parse([
                "check".into(),
                "--path".into(),
                "config.toml".into(),
                "--policy".into(),
                "trusted-config".into()
            ]),
            Ok(Command::Check(CheckOptions {
                profiles: vec![],
                path: Some("config.toml".into()),
                policy: Some(Policy::TrustedConfig)
            }))
        );
        assert!(parse(["check".into(), "--path".into(), "config.toml".into()]).is_err());
    }
}
