use anyhow::{anyhow, Result};
use std::path::Path;
use std::sync::Arc;

use crate::apps::App;
use crate::archive::ArchiveExtractor;
use crate::github::GithubClient;
use crate::types::{AppBinary, DownloadedAssets, ManPage};
use crate::version::AppVersion;

pub struct Zoxide { client: Arc<GithubClient> }

impl Zoxide {
    const OWNER: &'static str = "ajeetdsouza";
    const REPO:  &'static str = "zoxide";
    pub fn new(client: Arc<GithubClient>) -> Self { Self { client } }
}

impl App for Zoxide {
    fn exe_name(&self) -> &str { "zoxide" }
    fn url(&self) -> &str { "https://github.com/ajeetdsouza/zoxide" }

    fn released_version(&self) -> Result<AppVersion> {
        self.client.latest_release(Self::OWNER, Self::REPO)?.version()
    }

    fn download(&self) -> Result<DownloadedAssets> {
        let release = self.client.latest_release(Self::OWNER, Self::REPO)?;
        let deb_name = release
            .asset_names()
            .into_iter()
            .find(|a| a.ends_with("_amd64.deb"))
            .ok_or_else(|| anyhow!("Can't find zoxide .deb asset"))?;

        let asset = self.client.download_asset(Self::OWNER, Self::REPO, &deb_name)?;
        let deb = ArchiveExtractor::new(&deb_name, asset.data);
        let xz_data = deb.extract("data.tar.xz")?;
        let inner = ArchiveExtractor::new("data.tar.xz", xz_data);
        let members = inner.members()?;

        let exe_path = members.iter()
            .find(|m| *m == "./usr/bin/zoxide")
            .cloned()
            .ok_or_else(|| anyhow!("Can't find ./usr/bin/zoxide"))?;
        let binary_data = inner.extract(&exe_path)?;

        let mut man_pages = Vec::new();
        for member in &members {
            if member.starts_with("./usr/share/man/man1/") && member.ends_with(".1.gz") {
                let fname = Path::new(member).file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or("")
                    .to_string();
                man_pages.push(ManPage::new(1, fname, inner.extract(member)?));
            }
        }

        Ok(DownloadedAssets {
            binary: Some(AppBinary::new("zoxide", binary_data)),
            man_pages,
            // zoxide init generates completions at shell init time; no static files needed
            ..Default::default()
        })
    }
}
