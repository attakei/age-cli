# gazer

gazer is my bumpversion tool (for training Rust programming).

## Installation

```
cargo install --git https://github.com/attakei-lab/gazer.git
```

## Usage

At first, you must generate configuration file -- `.gazer.toml`.
You should edit it for your current version, because generated file is set `current_version = "0.0.0"`.

```
gazer init
```

If you want to up next version, run `gazer update|major|minor|patch`.

* `update` accepts any semver.
* `major` is shortcut to major version updating of semver.
* `minor` is shortcut to minor version updating of semver.
* `patch` is shortcut to patch version updating of semver.

Example:

```console
# If current_version = "1.2.3"
> gazer major  # Update to 2.0.0
> gazer minor  # Update to 1.3.0
> gazer patch  # Update to 1.2.4
```

## Configuration file

### `current_version`

Version value that gazer manages.

### `files`

List of replace target for update versioning.

* `path`: File path of cwd.
* `search`: Replace target line. `{{current_version}}` will be replaced for current version.
* `replace`: Replaced text for target. `{{new_version}}` will be replaced for next version.
