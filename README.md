# binup

Downloads and installs bunch of cmdline utilities into `/usr/local` directly from
`GitHub` and `Codeberg` releases.

For some (most) downloaded apps, it additionally installs to `/usr/local/`:

- `man` pages
- `ZSH`, `Bash` and `Fish` completions

## How it works

For supported apps, downloads latest release (binary) from `GitHub` or `Codeberg` and
installs it into `/usr/local`.

Installing into `/usr/local` doesn't interfere with the rest of the system. Ie. you can
have `ripgrep` installed from both, official distro package and from `binup`: updating
any of them will not overwrite the other. Which one gets used when you call `ripgrep`
from your shell, depends on your `$PATH`. In most modern distros, stuff from
`/usr/local` has priority.

## Why?

Whenever I need to `ssh` to some new VM, I usually loose access to my favorite
collection of CLI tools. Sometimes `sudo apt install ...` or similar can help. Often
times: it can't.

`binup` always works ... though downloaded binaries may not 😎

The risk is acceptable `99.999%` of times.

## How to use it?

```sh
# install everything into /usr/local
binup

# install a subset
binup --apps rg --apps bat --apps fzf

# install the hand-picked minimal set
binup --minimal-set

# install into a different prefix (no sudo needed)
binup --prefix ~/.local

# list all supported app identifiers
binup list-apps-ids
```

`GitHub` applies rate limiting to unauthenticated API requests. Providing a token avoids
hitting those limits.

```sh
# prompt for GitHub token interactively (default)
binup --gh-token-source prompt

# load GitHub token from GITHUB_API_TOKEN env var or ~/.config/github/api_token
binup --gh-token-source load

# load Codeberg token from CODEBERG_API_TOKEN env var or
# ~/.config/codeberg/api_token (default)
binup --cb-token-source load

# prompt for Codeberg token interactively
binup --cb-token-source prompt
```

One side-effect is that it always uses `~/.cache/binup` for stuff downloaded from
`GitHub` and `Codeberg`.

## Non goals

- `binup` is not a fully blown package manager
- `binup` always installs latest available versions, which may not work on your current
  system, may be broken release, or whatever else: there is not way to select or pin
  version of installed binary
- there is no way to uninstall installed files (besides deleting all relevant
  directories in `/usr/local` and trying again)
- it supports Linux only; you may be able to make it work on some other systems, but it
  was never intended to be used for that
- it supports only `x86_64` architecture and will not even try to download other
  binaries

## Supported apps

- [aqua](https://github.com/aquaproj/aqua)
- [ast-grep](https://github.com/ast-grep/ast-grep)
- [atuin](https://github.com/atuinsh/atuin)
- [bat](https://github.com/sharkdp/bat)
- [caddy](https://github.com/caddyserver/caddy)
- [carapace](https://github.com/carapace-sh/carapace-bin)
- [chezmoi](https://github.com/twpayne/chezmoi)
- [d4s](https://github.com/jr-k/d4s)
- [dasel](https://github.com/TomWright/dasel)
- [delta](https://github.com/dandavison/delta)
- [difft](https://github.com/Wilfred/difftastic)
- [dockmate](https://github.com/shubh-io/DockMate)
- [dry](https://github.com/moncho/dry)
- [eza](https://github.com/eza-community/eza)
- [fd](https://github.com/sharkdp/fd)
- [fnm](https://github.com/Schniz/fnm)
- [fx](https://github.com/antonmedv/fx)
- [fzf](https://github.com/junegunn/fzf)
- [gitleaks](https://github.com/gitleaks/gitleaks)
- [go](https://go.dev/)
- [gojq](https://github.com/itchyny/gojq)
- [gonzo](https://github.com/control-theory/gonzo)
- [jid](https://github.com/simeji/jid)
- [jq](https://github.com/jqlang/jq)
- [jqp](https://github.com/noahgorstein/jqp)
- [lazydocker](https://github.com/jesseduffield/lazydocker)
- [lazygit](https://github.com/jesseduffield/lazygit)
- [lazyjournal](https://github.com/Lifailon/lazyjournal)
- [mdbook](https://github.com/rust-lang/mdBook)
- [mergiraf](https://codeberg.org/mergiraf/mergiraf)
- [mise](https://github.com/jdx/mise)
- [neovide](https://github.com/neovide/neovide)
- [rclone](https://github.com/rclone/rclone)
- [restish](https://github.com/rest-sh/restish)
- [rg](https://github.com/BurntSushi/ripgrep)
- [rust-analyzer](https://github.com/rust-lang/rust-analyzer)
- [sd](https://github.com/chmln/sd)
- [sk](https://github.com/skim-rs/skim)
- [starship](https://github.com/starship/starship)
- [stylua](https://github.com/JohnnyMorganz/stylua)
- [uv](https://github.com/astral-sh/uv)
- [xq](https://github.com/sibprogrammer/xq)
- [yq](https://github.com/mikefarah/yq)
- [zoxide](https://github.com/ajeetdsouza/zoxide)

[^1]: This had once been written in Python.
      Workflow that required deployment of Python to be able to deploy `binup` to be
      able to deploy various CLI utilities was not one of my brightest ideas. Luckily,
      Claude was able to rewrite whole thing in Rust so I was able to abandon version in
      Python. 😎
