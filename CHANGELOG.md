# Changelog

## v0.6.0 - 2024-04-06 (JST)

### Features

- Find parent configuration files.
- Support extra toml file for configuration.
  - `Cargo.toml` for Rust project.
  - `pyproject.toml` for Python project.

## v0.5.0 - 2024-03-22 (JST)

### Features

- Support multiline replacement.
- `init` command support `--preset` option to customize first `.age.toml`.

### Misc

- Rename project (binary name is not changed).
- Transfer repository into personarl space.
- Add management target by age.
- Update documents.
- Launch documentation site on https://age.attakei.dev/

## v0.4.0 - 2024-03-21 (JST)

### Breaking chances

- Remove `version` subcommand (use `--version` instead)

### Features

- Print error these cases:
  - Run `init` if configuration file is already exists.
  - Run except for `init` if configuration file is not exists or is invalid format.

## v0.3.0 - 2024-03-18 (JST)

### Features

- `search` and `replace` (of `[[files]]` in config) are supported context values. (#2)
- Add `now` about execute datetime into context values. (#2)

## v0.2.1 - 2024-03-15 (JST)

### Fixes

- Correct template for `age init` command.

## v0.2.0 - 2024-03-10 (JST)

### Breaking changes

- It is renamed application.
- Change replace search and target rule.
  It use `{{ ~ }}` instead of `{ ~ }`.

## v0.1.0 - 2024-03-10 (JST)

### Features

- Update managed version
- Replace value of version target sources
