use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use rutils_downloader::{
    install_apps, known_apps_identifiers, load_github_token, select_apps, DEFAULT_PREFIX,
};

#[derive(Parser)]
#[command(name = "rutils-downloader")]
#[command(about = "Installs or updates CLI utilities directly from GitHub releases")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Install prefix (e.g. /usr/local or ~/.local)
    #[arg(short = 'p', long, default_value = DEFAULT_PREFIX)]
    prefix: PathBuf,

    /// App(s) to install; may be repeated. Defaults to all apps.
    #[arg(short = 'a', long = "apps", value_name = "NAME")]
    apps: Vec<String>,

    /// Where to load GitHub API token from (prompt or load)
    #[arg(short = 't', long, default_value = "prompt",
          value_parser = ["prompt", "load"])]
    token_source: String,

    /// Install a hand-picked minimal set of apps (overrides --apps)
    #[arg(long, default_value_t = false)]
    minimal_set: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Print all supported app identifiers
    ListAppsIds,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info"),
    )
    .format(|buf, record| {
        use std::io::Write;
        writeln!(
            buf,
            "lvl={} app=installer msg={}",
            record.level(),
            record.args()
        )
    })
    .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::ListAppsIds) => {
            for id in known_apps_identifiers() {
                println!("{}", id);
            }
        }
        None => {
            log::info!("Installing into: {:?}", cli.prefix);
            let token = load_github_token(&cli.token_source)?;
            let selected = select_apps(&cli.apps, cli.minimal_set)?;
            let installed = install_apps(&cli.prefix, &selected, token)?;
            if !installed.is_empty() {
                println!("Installed files:");
                for path in installed {
                    println!("- {}", path.display());
                }
            }
        }
    }

    Ok(())
}
