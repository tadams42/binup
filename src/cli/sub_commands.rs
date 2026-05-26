use anyhow::Result;

use crate::apps::all_apps_identifiers;
use crate::installer::install_apps;
use crate::uninstaller::uninstall_apps;

use super::helpers::{
    load_or_prompt_codeberg_token, load_or_prompt_github_token, load_or_prompt_gitlab_token,
    select_apps,
};
use super::main_command::Cli;

pub fn list_apps_ids_command() {
    for id in all_apps_identifiers() {
        println!("{}", id);
    }
}

pub fn install_apps_command(cli: &Cli) -> Result<()> {
    log::info!("Installing into: {:?}", cli.prefix);
    let (gh_token, cb_token, gl_token) = if cli.offline {
        (None, None, None)
    } else {
        (
            load_or_prompt_github_token(&cli.gh_token_source)?,
            load_or_prompt_codeberg_token(&cli.cb_token_source)?,
            load_or_prompt_gitlab_token(&cli.gl_token_source)?,
        )
    };
    let selected = select_apps(&cli.apps, cli.minimal_set)?;
    let installed = install_apps(&cli.prefix, &selected, gh_token, cb_token, gl_token, cli.offline)?;
    if !installed.is_empty() {
        println!("Installed files:");
        for path in installed {
            println!("- {}", path.display());
        }
    }

    Ok(())
}

pub fn uninstall_command(cli: &Cli) -> Result<()> {
    let selected = select_apps(&cli.apps, cli.minimal_set)?;
    let validated = select_apps(&selected, false)?;

    let removed = uninstall_apps(&cli.prefix, &validated)?;
    if removed.is_empty() {
        println!("No files removed.");
    } else {
        println!("Removed files:");
        for path in removed {
            println!("- {}", path.display());
        }
    }
    Ok(())
}

pub fn reinstall_apps_command(cli: &Cli) -> Result<()> {
    let selected = select_apps(&cli.apps, cli.minimal_set)?;
    let removed = uninstall_apps(&cli.prefix, &selected)?;
    if removed.is_empty() {
        println!("No files removed.");
    } else {
        println!("Removed files:");
        for path in &removed {
            println!("- {}", path.display());
        }
    }
    log::info!("Reinstalling into: {:?}", cli.prefix);
    let (gh_token, cb_token, gl_token) = if cli.offline {
        (None, None, None)
    } else {
        (
            load_or_prompt_github_token(&cli.gh_token_source)?,
            load_or_prompt_codeberg_token(&cli.cb_token_source)?,
            load_or_prompt_gitlab_token(&cli.gl_token_source)?,
        )
    };
    let installed = install_apps(&cli.prefix, &selected, gh_token, cb_token, gl_token, cli.offline)?;
    if !installed.is_empty() {
        println!("Installed files:");
        for path in installed {
            println!("- {}", path.display());
        }
    }

    Ok(())
}
