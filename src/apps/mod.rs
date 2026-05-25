use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::installer::install_assets;
use crate::types::DownloadedAssets;
use crate::version::AppVersion;

mod chezmoi;
mod containers;
mod data;
mod dev_envs;
mod dev_tools;
mod files;
mod git;
mod http;
mod logs;
mod rclone;
mod shell;

pub use chezmoi::Chezmoi;
pub use containers::{D4S, DockMate, Dry, LazyDocker};
pub use data::{Dasel, Fx, GoJq, Jid, Jq, Jqp, Qsv, QsvAll, Rsv, Xq, Yq};
pub use dev_envs::{Aqua, Fnm, Mise, Uv};
pub use dev_tools::{AstGrep, Mdbook, Neovide, RustAnalyzer, Stylua};
pub use files::{Bat, Dust, Eza, FdFind, Ripgrep, SdEdit, Yazi};
pub use git::{Delta, Difftastic, Gitleaks, Lazygit, Mergiraf};
pub use http::{Caddy, Restish, Xh};
pub use logs::{Gonzo, LazyJournal};
pub use rclone::Rclone;
pub use shell::{Atuin, Carapace, Fzf, Skim, Starship, Zoxide};

// ── App trait ────────────────────────────────────────────────────────────────

const DEFAULT_VERSION_ARG: &str = "--version";

pub trait App {
    fn exe_name(&self) -> &str;

    fn cli_version_arg(&self) -> &str { DEFAULT_VERSION_ARG }

    fn released_version(&self) -> Result<AppVersion>;

    fn download(&self) -> Result<DownloadedAssets>;

    fn installed_version(&self, prefix: &Path) -> Result<Option<AppVersion>> {
        let bin = prefix.join("bin").join(self.exe_name());
        if !bin.exists() {
            return Ok(None);
        }
        let out = std::process::Command::new(&bin)
            .arg(self.cli_version_arg())
            .output();
        match out {
            Err(_) => Ok(None),
            Ok(o) => {
                let stdout = String::from_utf8_lossy(&o.stdout);
                let stderr = String::from_utf8_lossy(&o.stderr);
                let combined = format!("{}{}", stdout, stderr);
                Ok(AppVersion::find_in(&combined))
            }
        }
    }

    fn needs_install(&self, prefix: &Path) -> Result<bool> {
        let installed = self.installed_version(prefix)?;
        match installed {
            None => Ok(true),
            Some(iv) => Ok(iv != self.released_version()?),
        }
    }

    fn install(&self, prefix: &Path) -> Result<Vec<PathBuf>> {
        if !self.needs_install(prefix)? {
            log::info!("lvl=INFO app={} msg=Already at latest version", self.exe_name());
            return Ok(vec![]);
        }

        let assets = self.download()?;
        let installed = install_assets(prefix, &assets)?;
        log::info!("lvl=INFO app={} msg=Installed", self.exe_name());
        Ok(installed)
    }
}

// ── Registry ─────────────────────────────────────────────────────────────────

pub struct AppEntry {
    pub id:          &'static str,
    pub url:         &'static str,
    pub category:    &'static str,
    pub description: &'static str,
}

static ALL_APP_ENTRIES: &[AppEntry] = &[
    AppEntry {
        id:          Aqua::ID,
        url:         Aqua::URL,
        category:    "dev_envs",
        description: Aqua::DESCRIPTION,
    },
    AppEntry {
        id:          AstGrep::ID,
        url:         AstGrep::URL,
        category:    "dev_tools",
        description: AstGrep::DESCRIPTION,
    },
    AppEntry {
        id:          Atuin::ID,
        url:         Atuin::URL,
        category:    "shell",
        description: Atuin::DESCRIPTION,
    },
    AppEntry {
        id:          Bat::ID,
        url:         Bat::URL,
        category:    "files",
        description: Bat::DESCRIPTION,
    },
    AppEntry {
        id:          Caddy::ID,
        url:         Caddy::URL,
        category:    "http",
        description: Caddy::DESCRIPTION,
    },
    AppEntry {
        id:          Carapace::ID,
        url:         Carapace::URL,
        category:    "shell",
        description: Carapace::DESCRIPTION,
    },
    AppEntry {
        id:          Chezmoi::ID,
        url:         Chezmoi::URL,
        category:    "other",
        description: Chezmoi::DESCRIPTION,
    },
    AppEntry {
        id:          D4S::ID,
        url:         D4S::URL,
        category:    "containers",
        description: D4S::DESCRIPTION,
    },
    AppEntry {
        id:          Dasel::ID,
        url:         Dasel::URL,
        category:    "data",
        description: Dasel::DESCRIPTION,
    },
    AppEntry {
        id:          Delta::ID,
        url:         Delta::URL,
        category:    "git",
        description: Delta::DESCRIPTION,
    },
    AppEntry {
        id:          Difftastic::ID,
        url:         Difftastic::URL,
        category:    "git",
        description: Difftastic::DESCRIPTION,
    },
    AppEntry {
        id:          DockMate::ID,
        url:         DockMate::URL,
        category:    "containers",
        description: DockMate::DESCRIPTION,
    },
    AppEntry {
        id:          Dry::ID,
        url:         Dry::URL,
        category:    "containers",
        description: Dry::DESCRIPTION,
    },
    AppEntry {
        id:          Dust::ID,
        url:         Dust::URL,
        category:    "files",
        description: Dust::DESCRIPTION,
    },
    AppEntry {
        id:          Eza::ID,
        url:         Eza::URL,
        category:    "files",
        description: Eza::DESCRIPTION,
    },
    AppEntry {
        id:          FdFind::ID,
        url:         FdFind::URL,
        category:    "files",
        description: FdFind::DESCRIPTION,
    },
    AppEntry {
        id:          Fnm::ID,
        url:         Fnm::URL,
        category:    "dev_envs",
        description: Fnm::DESCRIPTION,
    },
    AppEntry {
        id:          Fx::ID,
        url:         Fx::URL,
        category:    "data",
        description: Fx::DESCRIPTION,
    },
    AppEntry {
        id:          Fzf::ID,
        url:         Fzf::URL,
        category:    "shell",
        description: Fzf::DESCRIPTION,
    },
    AppEntry {
        id:          Gitleaks::ID,
        url:         Gitleaks::URL,
        category:    "git",
        description: Gitleaks::DESCRIPTION,
    },
    AppEntry {
        id:          GoJq::ID,
        url:         GoJq::URL,
        category:    "data",
        description: GoJq::DESCRIPTION,
    },
    AppEntry {
        id:          Gonzo::ID,
        url:         Gonzo::URL,
        category:    "logs",
        description: Gonzo::DESCRIPTION,
    },
    AppEntry {
        id:          Jid::ID,
        url:         Jid::URL,
        category:    "data",
        description: Jid::DESCRIPTION,
    },
    AppEntry {
        id:          Jq::ID,
        url:         Jq::URL,
        category:    "data",
        description: Jq::DESCRIPTION,
    },
    AppEntry {
        id:          Jqp::ID,
        url:         Jqp::URL,
        category:    "data",
        description: Jqp::DESCRIPTION,
    },
    AppEntry {
        id:          LazyJournal::ID,
        url:         LazyJournal::URL,
        category:    "logs",
        description: LazyJournal::DESCRIPTION,
    },
    AppEntry {
        id:          LazyDocker::ID,
        url:         LazyDocker::URL,
        category:    "containers",
        description: LazyDocker::DESCRIPTION,
    },
    AppEntry {
        id:          Lazygit::ID,
        url:         Lazygit::URL,
        category:    "git",
        description: Lazygit::DESCRIPTION,
    },
    AppEntry {
        id:          Mdbook::ID,
        url:         Mdbook::URL,
        category:    "dev_tools",
        description: Mdbook::DESCRIPTION,
    },
    AppEntry {
        id:          Mergiraf::ID,
        url:         Mergiraf::URL,
        category:    "git",
        description: Mergiraf::DESCRIPTION,
    },
    AppEntry {
        id:          Mise::ID,
        url:         Mise::URL,
        category:    "dev_envs",
        description: Mise::DESCRIPTION,
    },
    AppEntry {
        id:          Neovide::ID,
        url:         Neovide::URL,
        category:    "dev_tools",
        description: Neovide::DESCRIPTION,
    },
    AppEntry {
        id:          Rclone::ID,
        url:         Rclone::URL,
        category:    "other",
        description: Rclone::DESCRIPTION,
    },
    AppEntry {
        id:          Restish::ID,
        url:         Restish::URL,
        category:    "http",
        description: Restish::DESCRIPTION,
    },
    AppEntry {
        id:          Ripgrep::ID,
        url:         Ripgrep::URL,
        category:    "files",
        description: Ripgrep::DESCRIPTION,
    },
    AppEntry {
        id:          Qsv::ID,
        url:         Qsv::URL,
        category:    "data",
        description: Qsv::DESCRIPTION,
    },
    AppEntry {
        id:          QsvAll::ID,
        url:         QsvAll::URL,
        category:    "data",
        description: QsvAll::DESCRIPTION,
    },
    AppEntry {
        id:          Rsv::ID,
        url:         Rsv::URL,
        category:    "data",
        description: Rsv::DESCRIPTION,
    },
    AppEntry {
        id:          RustAnalyzer::ID,
        url:         RustAnalyzer::URL,
        category:    "dev_tools",
        description: RustAnalyzer::DESCRIPTION,
    },
    AppEntry {
        id:          SdEdit::ID,
        url:         SdEdit::URL,
        category:    "files",
        description: SdEdit::DESCRIPTION,
    },
    AppEntry {
        id:          Skim::ID,
        url:         Skim::URL,
        category:    "shell",
        description: Skim::DESCRIPTION,
    },
    AppEntry {
        id:          Starship::ID,
        url:         Starship::URL,
        category:    "shell",
        description: Starship::DESCRIPTION,
    },
    AppEntry {
        id:          Stylua::ID,
        url:         Stylua::URL,
        category:    "dev_tools",
        description: Stylua::DESCRIPTION,
    },
    AppEntry {
        id:          Uv::ID,
        url:         Uv::URL,
        category:    "dev_envs",
        description: Uv::DESCRIPTION,
    },
    AppEntry {
        id:          Xh::ID,
        url:         Xh::URL,
        category:    "http",
        description: Xh::DESCRIPTION,
    },
    AppEntry {
        id:          Xq::ID,
        url:         Xq::URL,
        category:    "data",
        description: Xq::DESCRIPTION,
    },
    AppEntry {
        id:          Yazi::ID,
        url:         Yazi::URL,
        category:    "files",
        description: Yazi::DESCRIPTION,
    },
    AppEntry {
        id:          Yq::ID,
        url:         Yq::URL,
        category:    "data",
        description: Yq::DESCRIPTION,
    },
    AppEntry {
        id:          Zoxide::ID,
        url:         Zoxide::URL,
        category:    "shell",
        description: Zoxide::DESCRIPTION,
    },
];

pub fn all_app_entries() -> &'static [AppEntry] { ALL_APP_ENTRIES }

pub fn create_app(
    id: &str, gh_token: Option<String>, cb_token: Option<String>, offline: bool,
) -> Option<Box<dyn App>> {
    use crate::codeberg::CodebergClient;
    use crate::github::GithubClient;
    use std::sync::Arc;
    let client = Arc::new(GithubClient::new(gh_token, offline));
    match id {
        Aqua::ID => Some(Box::new(Aqua::new(client))),
        AstGrep::ID => Some(Box::new(AstGrep::new(client))),
        Atuin::ID => Some(Box::new(Atuin::new(client))),
        Bat::ID => Some(Box::new(Bat::new(client))),
        Caddy::ID => Some(Box::new(Caddy::new(client))),
        Carapace::ID => Some(Box::new(Carapace::new(client))),
        chezmoi::Chezmoi::ID => Some(Box::new(Chezmoi::new(client))),
        D4S::ID => Some(Box::new(D4S::new(client))),
        Dasel::ID => Some(Box::new(Dasel::new(client))),
        Delta::ID => Some(Box::new(Delta::new(client))),
        Difftastic::ID => Some(Box::new(Difftastic::new(client))),
        DockMate::ID => Some(Box::new(DockMate::new(client))),
        Dry::ID => Some(Box::new(Dry::new(client))),
        Dust::ID => Some(Box::new(Dust::new(client))),
        Eza::ID => Some(Box::new(Eza::new(client))),
        FdFind::ID => Some(Box::new(FdFind::new(client))),
        Fnm::ID => Some(Box::new(Fnm::new(client))),
        Fx::ID => Some(Box::new(Fx::new(client))),
        Fzf::ID => Some(Box::new(Fzf::new(client))),
        Gitleaks::ID => Some(Box::new(Gitleaks::new(client))),
        GoJq::ID => Some(Box::new(GoJq::new(client))),
        Gonzo::ID => Some(Box::new(Gonzo::new(client))),
        Jid::ID => Some(Box::new(Jid::new(client))),
        Jq::ID => Some(Box::new(Jq::new(client))),
        Jqp::ID => Some(Box::new(Jqp::new(client))),
        LazyJournal::ID => Some(Box::new(LazyJournal::new(client))),
        LazyDocker::ID => Some(Box::new(LazyDocker::new(client))),
        Lazygit::ID => Some(Box::new(Lazygit::new(client))),
        Mdbook::ID => Some(Box::new(Mdbook::new(client))),
        Mergiraf::ID => {
            Some(Box::new(Mergiraf::new(Arc::new(CodebergClient::new(
                cb_token, offline,
            )))))
        }
        Mise::ID => Some(Box::new(Mise::new(client))),
        Neovide::ID => Some(Box::new(Neovide::new(client))),
        Rclone::ID => Some(Box::new(Rclone::new(client))),
        Restish::ID => Some(Box::new(Restish::new(client))),
        Ripgrep::ID => Some(Box::new(Ripgrep::new(client))),
        Qsv::ID => Some(Box::new(Qsv::new(client))),
        QsvAll::ID => Some(Box::new(QsvAll::new(client))),
        Rsv::ID => Some(Box::new(Rsv::new(client))),
        RustAnalyzer::ID => Some(Box::new(RustAnalyzer::new(client))),
        SdEdit::ID => Some(Box::new(SdEdit::new(client))),
        Skim::ID => Some(Box::new(Skim::new(client))),
        Starship::ID => Some(Box::new(Starship::new(client))),
        Stylua::ID => Some(Box::new(Stylua::new(client))),
        Uv::ID => Some(Box::new(Uv::new(client))),
        Xh::ID => Some(Box::new(Xh::new(client))),
        Xq::ID => Some(Box::new(Xq::new(client))),
        Yazi::ID => Some(Box::new(Yazi::new(client))),
        Yq::ID => Some(Box::new(Yq::new(client))),
        Zoxide::ID => Some(Box::new(Zoxide::new(client))),
        _ => None,
    }
}
