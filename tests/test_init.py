"""Test case for ``age init``."""

from pathlib import Path
from subprocess import PIPE, run


def test_it(target_bin: str, tmp_path: Path):
    """'init' command generate config file."""
    proc = run([target_bin, "init"], stdout=PIPE, stderr=PIPE, text=True, cwd=tmp_path)
    assert proc.returncode == 0
    assert (tmp_path / ".age.toml").exists()
