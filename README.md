# Simple commits

<div align="center">

[![Check CI](https://github.com/romancitodev/simple-commits/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/romancitodev/simple-commits/actions/workflows/checks.yml)

</div>

A little CLI written in rust to improve your dirty commits into **conventional** ones.

## 👀 Demo _(coming soon)_

## ✨ Features

- Fully conventional commits
- Auto-commit
- Custom templates
- Written in rust

## 📥 Installation

Install it using cargo!

```bash
cargo install simple-commit
```
Nixos

```bash
nix profile install github:romancitodev/simple-commits
```

## 🛠 Configuration

in your `~/$CONFIG_FOLDER` create a `sc` directory with a `config.toml` inside.

> [!TIP]
>
> ```bash
> mkdir ~/$CONFIG_FOLDER/sc && touch ~/$CONFIG_FOLDER/sc/config.toml
> ```

and use this template to configure it as you want.

```toml
# The scopes
[[scopes]]
name = "app"
description = "the app itself"

[[scopes]]
name = "core"
description = "the core lib"

[git]
# By default the skip preview flag is setted to false because we know
# It's a dangerous action.
skip_preview = true

skip_emoji = true

# Customize your commit template as you want
commit_template = ["git", "commit", "-m", "{{message}}", "&&", "git", "push"]
```

## 💻 Usage

To use it you just need to run one command. 😍

```bash
sc
```

or if you prefer to want to use flags:

| flags                       | Description                                  |
| --------------------------- | -------------------------------------------- |
| `-p` \| `--skip-preview`    | ⚠️ Skips the preview step (Dangerous)        |
| `-e` \| `--skip-emoji`      | Skips the emoji step                         |
| `-c` \| `--commit-template` | Command to run after generate commit message |
| `--config`                  | Set the config path                          |
