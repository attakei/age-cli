"""Test case for ``age info``."""

import textwrap
from pathlib import Path
from subprocess import PIPE, run


def test_not_configured_env(target_bin: str, tmp_path: Path):
    """'info' command requires configuration file.

    If age does not find configuration file, it should display message.
    """
    proc = run([target_bin, "info"], stdout=PIPE, stderr=PIPE, text=True, cwd=tmp_path)
    assert proc.returncode == 1
    assert "not exists." in proc.stderr


def test_empty_conf(target_bin: str, tmp_path: Path):  # noqa: D103
    (tmp_path / ".age.toml").touch()
    proc = run([target_bin, "info"], stdout=PIPE, stderr=PIPE, text=True, cwd=tmp_path)
    assert proc.returncode == 1


def test_invalid_conf(target_bin: str, tmp_path: Path):  # noqa: D103
    conf_text = textwrap.dedent(
        """\
    current_version = "0.0.0"
    """
    )
    (tmp_path / ".age.toml").write_text(conf_text)

    proc = run([target_bin, "info"], stdout=PIPE, stderr=PIPE, text=True, cwd=tmp_path)
    assert proc.returncode == 1
