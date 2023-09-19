use clap::{Parser, ValueEnum};

/// Searches and plays local music files
/// with Regex!!  
///
/// Built on top of locate.
/// - Locate must be installed
/// - Locate database must be updated with `updatedb`
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opt {
    /// Include files that don't end in .wav, .mp3 or .ogg
    #[arg(short, long, default_value_t = false)]
    pub all: bool,

    /// Ignore case in regular expressions
    #[arg(short = 'i', long = "insensitive", default_value_t = false)]
    pub case_insensitive: bool,

    /// Defines how arguments should be parsed
    #[arg(short, long = "mode", default_value = "all")]
    pub matchmode: MatchMode,

    /// Loop over all elements
    #[arg(short = 'l', long = "loop", default_value_t = false)]
    pub loop_all: bool,

    /// Repeat individual elements
    #[arg(short = 'r', long = "repeat", default_value_t = false)]
    pub repeat_individual: bool,

    /// Regular expressions to search with
    pub expressions: Vec<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum MatchMode {
    All,
    Any,
}
