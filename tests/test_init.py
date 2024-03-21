"""Test case for ``age init``."""

from pathlib import Path

import pytest


def test_it(cmd, tmp_path: Path):
    """'init' command generate config file."""
    proc = cmd("init")
    assert proc.returncode == 0
    #
    age_toml = tmp_path / ".age.toml"
    assert age_toml.exists()
    age_text = age_toml.read_text()
    assert 'current_version = "0.0.0"' in age_text


@pytest.mark.parametrize(
    "presets,files",
    [
        (["rust"], ["Cargo.toml"]),
        (["python"], ["pyproject.toml"]),
        (["rust", "python"], ["Cargo.toml", "pyproject.toml"]),
    ],
)
def test_with_preset(cmd, tmp_path: Path, presets: list[str], files: list[str]):
    """'init' command generate config file."""
    args = ["init", "--preset"] + presets
    proc = cmd(*args)
    assert proc.returncode == 0
    #
    age_toml = tmp_path / ".age.toml"
    assert age_toml.exists()
    age_text = age_toml.read_text()
    assert 'current_version = "0.0.0"' in age_text
    for f in files:
        assert f'[[files]]\npath = "{f}"' in age_text
