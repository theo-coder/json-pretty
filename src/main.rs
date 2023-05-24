use clap::{crate_authors, crate_version, Parser};
use json_pretty::{level::LogLevel, sources::process_stdin};

#[derive(Parser, Debug)]
#[command(version = crate_version!(), author = crate_authors!())]
struct Cli {
    /// Only show messages at or above the specified level.
    ///
    /// You can specify level names (trace, debug, info, warn, error, fatal)
    #[clap(short, long, value_enum, default_value = "trace")]
    level: LogLevel,
    /// Colorize output.
    ///
    /// Defaults to try if output stream is a TTY.
    #[arg(long = "color", conflicts_with = "no-color")]
    color: bool,
    /// Force no coloring (e.g. terminal doesn't support it).
    #[arg(name = "no-color", long = "no-color", conflicts_with = "color")]
    no_color: bool,
}

fn main() {
    let args = Cli::parse();

    if args.no_color {
        colored::control::set_override(false);
    } else if args.color || atty::is(atty::Stream::Stdout) {
        colored::control::set_override(true);
    }

    process_stdin(args.level);
}
