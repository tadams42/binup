use anyhow::{anyhow, Result};
use std::path::Path;
use std::sync::Arc;

use crate::apps::App;
use crate::archive::ArchiveExtractor;
use crate::github::GithubClient;
use crate::installer::{gen_completions_with_flags, with_temp_exe, run_cmd};
use crate::types::{AppBinary, Completion, DownloadedAssets, ManPage, Shell};
use crate::version::AppVersion;

pub struct Ripgrep { client: Arc<GithubClient> }

impl Ripgrep {
    const OWNER: &'static str = "BurntSushi";
    const REPO:  &'static str = "ripgrep";
    pub fn new(client: Arc<GithubClient>) -> Self { Self { client } }
}

impl App for Ripgrep {
    fn exe_name(&self) -> &str { "rg" }
    fn url(&self) -> &str { "https://github.com/BurntSushi/ripgrep" }
    fn installed_version_word_index(&self) -> isize { 1 }

    fn released_version(&self) -> Result<AppVersion> {
        self.client.latest_release(Self::OWNER, Self::REPO)?.version()
    }

    fn download(&self) -> Result<DownloadedAssets> {
        let release = self.client.latest_release(Self::OWNER, Self::REPO)?;
        let deb_name = release
            .asset_names()
            .into_iter()
            .find(|a| a.starts_with("ripgrep_") && a.ends_with("_amd64.deb"))
            .ok_or_else(|| anyhow!("Can't find ripgrep .deb asset"))?;

        let asset = self.client.download_asset(Self::OWNER, Self::REPO, &deb_name)?;
        let deb = ArchiveExtractor::new(&deb_name, asset.data);
        let xz_data = deb.extract("data.tar.xz")?;
        let inner = ArchiveExtractor::new("data.tar.xz", xz_data);
        let members = inner.members()?;

        let exe_path = members.iter()
            .find(|m| *m == "./usr/bin/rg")
            .cloned()
            .ok_or_else(|| anyhow!("Can't find ./usr/bin/rg"))?;
        let binary_data = inner.extract(&exe_path)?;

        let man_path = members.iter()
            .find(|m| *m == "./usr/share/man/man1/rg.1.gz")
            .cloned()
            .ok_or_else(|| anyhow!("Can't find rg.1.gz"))?;
        let man_data = inner.extract(&man_path)?;

        let completions = with_temp_exe("rg", &binary_data, |exe| {
            Ok(vec![
                Completion::zsh("rg", run_cmd(exe, &["--generate", "complete-zsh"])?),
                Completion::bash("rg", run_cmd(exe, &["--generate", "complete-bash"])?),
                Completion::fish("rg", run_cmd(exe, &["--generate", "complete-fish"])?),
            ])
        })?;

        Ok(DownloadedAssets {
            binary: Some(AppBinary::new("rg", binary_data)),
            man_pages: vec![ManPage::new(1, "rg.1.gz", man_data)],
            completions,
            ..Default::default()
        })
    }
}
