use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::process::ExitCode;

use tokenkeeper::cli::{self, CheckOptions};
use tokenkeeper::inspector::MetadataInspector;
use tokenkeeper::profiles::{
    builtin_registry, LocationSpec, NodeKind, Platform, Policy, ProfileSpec, Root,
};
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
        Ok(cli::Command::Profiles) => list_profiles(),
        Ok(cli::Command::Check(options)) => run_check(options),
        Err(error) => {
            eprintln!("error: {error}");
            eprintln!("Run `tokenkeeper --help` for usage.");
            ExitCode::from(2)
        }
    }
}

fn list_profiles() -> ExitCode {
    for profile in builtin_registry().profiles() {
        let source = profile.source.as_deref().unwrap_or("unspecified");
        println!(
            "{}\t{}\tmacOS/Linux\tevidence={source}",
            profile.id, profile.display_name
        );
    }
    ExitCode::SUCCESS
}

fn run_check(options: CheckOptions) -> ExitCode {
    let home = match std::env::var_os("HOME") {
        Some(home) => PathBuf::from(home),
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
    let inspector = match MetadataInspector::new(&home, metadata.uid()) {
        Ok(inspector) => inspector,
        Err(error) => {
            eprintln!("cannot initialize inspector: {error}");
            return ExitCode::from(2);
        }
    };
    if let Some(path) = options.path {
        let Some(policy) = options.policy else {
            return ExitCode::from(2);
        };
        let relative = match path.strip_prefix(&home) {
            Ok(relative) if !relative.as_os_str().is_empty() => relative,
            _ => {
                eprintln!("--path must be inside HOME and must not be HOME itself");
                return ExitCode::from(2);
            }
        };
        let location = LocationSpec::exact(Root::Home, relative, NodeKind::Either, policy, false);
        return inspect_locations(&inspector, [("custom", &location, policy)]);
    }
    let registry = builtin_registry();
    let selected: Vec<&ProfileSpec> = if options.profiles.is_empty() {
        registry.profiles().iter().collect()
    } else {
        let mut selected = Vec::new();
        for id in &options.profiles {
            match registry.find(id) {
                Some(profile) => selected.push(profile),
                None => {
                    eprintln!("unknown profile `{id}`");
                    return ExitCode::from(2);
                }
            }
        }
        selected
    };
    let mut summary = Summary::default();
    for profile in selected {
        if !profile.platforms.contains(&Platform::MacOs) {
            continue;
        }
        for location in &profile.locations {
            match inspector.inspect_location(location) {
                Ok(results) => {
                    for result in results {
                        summary.add(&result);
                        print!("{}: {}", profile.id, render(&result, Some(location.policy)));
                    }
                }
                Err(error) => {
                    eprintln!("{}: inspection failed: {error}", profile.id);
                    return ExitCode::from(2);
                }
            }
        }
    }
    println!("{}", tokenkeeper::report::summary_line(summary));
    ExitCode::from(summary.exit_code())
}

fn inspect_locations<'a, I>(inspector: &MetadataInspector, locations: I) -> ExitCode
where
    I: IntoIterator<Item = (&'a str, &'a LocationSpec, Policy)>,
{
    let mut summary = Summary::default();
    for (label, location, policy) in locations {
        match inspector.inspect_location(location) {
            Ok(results) => {
                for result in results {
                    summary.add(&result);
                    print!("{label}: {}", render(&result, Some(policy)));
                }
            }
            Err(error) => {
                eprintln!("inspection failed: {error}");
                return ExitCode::from(2);
            }
        }
    }
    println!("{}", tokenkeeper::report::summary_line(summary));
    ExitCode::from(summary.exit_code())
}
