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
    /// Pool of possible results
    #[arg(short, long, default_value = r"\.(mp3|ogg|wav)$")]
    pub pool: String,

    /// Ignore case in regular expressions
    #[arg(short = 'i', long = "insensitive", default_value_t = false)]
    pub case_insensitive: bool,

    /// Set how files must match given expressions
    #[arg(short, long = "match", default_value = "any")]
    pub matchmode: MatchMode,

    /// Output mode for results
    #[arg(short, long, default_value = "tree")]
    pub output: Output,

    /// Loop over all elements
    #[arg(short = 'l', long = "loop", default_value_t = false)]
    pub loop_all: bool,

    /// Repeat individual elements
    #[arg(short = 'r', long = "repeat", default_value_t = false)]
    pub repeat_individual: bool,

    /// Regular expressions to search with
    pub expressions: Vec<String>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum MatchMode {
    /// Results must match all expressions
    All,
    /// Results must match any expression
    Any,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Output {
    /// List full paths of results (useful for passing to other programs)
    List,
    /// Display a compact tree graph of results
    Tree,
}
