"""Test case for ``age info``."""

import shutil
import textwrap
from pathlib import Path
from subprocess import PIPE, CompletedProcess, run

import pytest

from . import get_env_dirs


def test_not_configured_env(target_bin: str, tmp_path: Path):
    """'info' command requires configuration file.

    If age does not find configuration file, it should display message.
    """
    proc = run([target_bin, "info"], stdout=PIPE, stderr=PIPE, text=True, cwd=tmp_path)
    assert proc.returncode == 1
    assert "Workspace is not found." in proc.stderr


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


@pytest.mark.parametrize(
    "env_path",
    **get_env_dirs("return-0"),
)
def test_valid_env(cmd, env_path: Path, tmp_path: Path):
    """Run test cases on env having valid files."""
    shutil.copytree(env_path / "before", tmp_path, dirs_exist_ok=True)
    proc: CompletedProcess = cmd("info")
    assert proc.returncode == 0


@pytest.mark.parametrize(
    "env_path",
    **get_env_dirs("return-0"),
)
def test_valid_env_on_child(cmd, env_path: Path, tmp_path: Path):
    """Run test cases on env having valid files."""
    shutil.copytree(env_path / "before", tmp_path, dirs_exist_ok=True)
    work_dir = tmp_path / "working_directory"
    work_dir.mkdir()
    proc: CompletedProcess = cmd("info", cwd=work_dir)
    assert proc.returncode == 0
