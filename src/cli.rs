use std::ffi::OsString;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    Version,
}

pub const HELP: &str = "Usage: tokenkeeper [OPTIONS]\n\nOptions:\n  -h, --help     Show this help message\n  -V, --version  Show the version\n";

pub fn parse<I>(args: I) -> Result<Command, String>
where
    I: IntoIterator<Item = OsString>,
{
    let mut args = args.into_iter();
    let Some(first) = args.next() else {
        return Ok(Command::Help);
    };

    let command = match first.to_str() {
        Some("-h") | Some("--help") => Command::Help,
        Some("-V") | Some("--version") => Command::Version,
        _ => return Err(format!("unknown argument `{}`", first.to_string_lossy())),
    };

    if let Some(extra) = args.next() {
        return Err(format!("unexpected argument `{}`", extra.to_string_lossy()));
    }

    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::{parse, Command};
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
}
