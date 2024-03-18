"""Test case for ``age version``."""

import tomllib
from pathlib import Path
from subprocess import PIPE, run


def test_it(root: Path, target_bin: str):  # noqa: D103
    cargo = tomllib.loads((root / "Cargo.toml").read_text())
    proc = run([target_bin, "version"], stdout=PIPE, stderr=PIPE, text=True)
    assert proc.returncode == 0
    assert f"v{cargo['package']['version']}" in proc.stdout
