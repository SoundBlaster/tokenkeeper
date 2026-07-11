use std::os::unix::fs::MetadataExt;
use std::process::ExitCode;

use tokenkeeper::cli::{self, CheckOptions};
use tokenkeeper::inspector::MetadataInspector;
use tokenkeeper::profiles::{LocationSpec, NodeKind, Root};
use tokenkeeper::report::{render, Summary};

fn main() -> ExitCode {
    match cli::parse(std::env::args_os().skip(1)) {
        Ok(cli::Command::Help) => {
            print!("{}", cli::HELP);
            ExitCode::SUCCESS
        }
        Ok(cli::Command::Version) => {
            println!("tokenkeeper {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Ok(cli::Command::Profiles) => {
            println!("No built-in profiles are installed yet.");
            ExitCode::SUCCESS
        }
        Ok(cli::Command::Check(options)) => run_check(options),
        Err(error) => {
            eprintln!("error: {error}");
            eprintln!("Run `tokenkeeper --help` for usage.");
            ExitCode::from(2)
        }
    }
}

fn run_check(options: CheckOptions) -> ExitCode {
    if !options.profiles.is_empty() {
        eprintln!("profile checks are not available until built-in profiles are installed");
        return ExitCode::from(2);
    }
    let Some(path) = options.path else {
        eprintln!("no check scope supplied; use --path PATH --policy POLICY or select a profile");
        return ExitCode::from(2);
    };
    let Some(policy) = options.policy else {
        return ExitCode::from(2);
    };
    let home = match std::env::var_os("HOME") {
        Some(home) => std::path::PathBuf::from(home),
        None => {
            eprintln!("HOME is not set");
            return ExitCode::from(2);
        }
    };
    let metadata = match std::fs::symlink_metadata(&home) {
        Ok(metadata) => metadata,
        Err(error) => {
            eprintln!("cannot inspect HOME: {error}");
            return ExitCode::from(2);
        }
    };
    let relative = match path.strip_prefix(&home) {
        Ok(relative) if !relative.as_os_str().is_empty() => relative,
        _ => {
            eprintln!("--path must be inside HOME and must not be HOME itself");
            return ExitCode::from(2);
        }
    };
    let location = LocationSpec::exact(Root::Home, relative, NodeKind::Either, policy, false);
    let inspector = match MetadataInspector::new(&home, metadata.uid()) {
        Ok(inspector) => inspector,
        Err(error) => {
            eprintln!("cannot initialize inspector: {error}");
            return ExitCode::from(2);
        }
    };
    let results = match inspector.inspect_location(&location) {
        Ok(results) => results,
        Err(error) => {
            eprintln!("inspection failed: {error}");
            return ExitCode::from(2);
        }
    };
    let mut summary = Summary::default();
    for result in &results {
        summary.add(result);
        print!("{}", render(result, Some(policy)));
    }
    println!("{}", tokenkeeper::report::summary_line(summary));
    ExitCode::from(summary.exit_code())
}
