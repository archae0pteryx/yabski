use clap::{Args, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(value_enum)]
    pub what: What,

    #[arg(value_enum)]
    pub with: With,

    pub where_: Option<String>,

    pub to: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum What {
    Query,
    Create,
    Focus,
    Destroy,
    Label
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum With {
    Space,
    Spaces,
    Window,
    Windows,
    Display,
    Displays,
}
