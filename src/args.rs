use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help = true)]
#[clap(group(ArgGroup::new("input").required(true).multiple(true)))]
#[clap(group(ArgGroup::new("output").multiple(false)))]
pub(crate) struct Cli {
    /// Download largest torrent instead of downloading
    #[arg(long, short, group = "output")]
    pub(crate) torrent: bool,

    /// Print magnet link of largest torrent instead of downloading
    #[arg(long, short, group = "output")]
    pub(crate) magnet: bool,

    /// Download Favorites
    #[arg(long, short, group = "input")]
    pub(crate) favorites: bool,

    /// URLs to download
    #[arg(trailing_var_arg = true, group = "input")]
    pub(crate) urls: Vec<String>,
}
