"""Test case for ``age info``."""

from pathlib import Path
from subprocess import PIPE, run


def test_not_configured_env(target_bin: str, tmp_path: Path):
    """'info' command requires configuration file.

    If age does not find configuration file, it should display message.
    """
    proc = run([target_bin, "info"], stdout=PIPE, stderr=PIPE, text=True, cwd=tmp_path)
    assert proc.returncode == 1
    assert "not exists." in proc.stderr
