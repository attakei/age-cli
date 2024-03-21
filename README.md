# age

age is my bumpversion tool (for training Rust programming).

![GitHub Tag](https://img.shields.io/github/v/tag/attakei/age-cli)
[![CI jobs](https://github.com/attakei/age-cli/actions/workflows/main.yml/badge.svg)](https://github.com/attakei/age-cli/actions/workflows/main.yml)

## Overview

This is faster bump-version tools inspired by bumpversion and inherits.
**age** is a meaning that *logging lib's birthday* and *"Up" in Japanese slang*.

## Installation

```
cargo install --git https://github.com/attakei/age-cli.git
```

There are pre-build binaries on [GitHub releases](https://github.com/attakei/age-cli/releases).
You can download and use it directly without Rust environment.

## Usage

At first, you must generate configuration file -- `.age.toml`.
You should edit it for your current version, because generated file is set `current_version = "0.0.0"`.

```
age init
```

If you want to up next version, run `age update|major|minor|patch`.

* `update` accepts any semver.
* `major` is shortcut to major version updating of semver.
* `minor` is shortcut to minor version updating of semver.
* `patch` is shortcut to patch version updating of semver.

Example:

```console
# If current_version = "1.2.3"
> age major  # Update to 2.0.0
> age minor  # Update to 1.3.0
> age patch  # Update to 1.2.4
```

## Configuration file

### `current_version`

Version value that `age` manages.

### `files`

List of replace target for update versioning.

* `path`: File path of cwd.
* `search`: Replace target line.
* `replace`: Replaced text for target.

## Templating context

Age uses template-engine to search and replace targets.
You can set context values into `files.search` and `files.replace`.

If you want to know example, please see [.age.toml](https://github.com/attakei/age-cli/blob/main/.age.toml).

### Context values.

- `current_version`: Version text before run command.
- `new_version`: New version text after run command.
- `now`: Date time text when context is created (as ISO 8601 format string).
