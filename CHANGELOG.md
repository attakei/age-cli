# Changelog

## v0.4.0 - 2024-03-21 (JST)

## Breaking chances

- Remove `version` subcommand (use `--version` intead)

### Features

- Print error these cases:
  - Run `init` if configuration file is already exists.
  - Run except for `init` if configuration file is not exists or is invalid foramt.

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
