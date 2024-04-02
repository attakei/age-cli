"""Test case for ``age update``."""

import shutil
from pathlib import Path
from subprocess import CompletedProcess, run

import pytest

from . import get_env_dirs


def test_invalid_args(cmd, tmp_path: Path):
    """Run test case on env having valid files, but command argument is invalid."""
    env_path = get_env_dirs("return-0")["argvalues"][0]
    shutil.copytree(env_path, tmp_path, dirs_exist_ok=True)
    proc: CompletedProcess = cmd("update", "-1")
    assert proc.returncode != 0


@pytest.mark.parametrize(
    "env_path",
    **get_env_dirs("return-1"),
)
def test_invalid_env(cmd, env_path: Path, tmp_path: Path):
    """Run test caese on env having invalid configuration."""
    shutil.copytree(env_path, tmp_path, dirs_exist_ok=True)
    proc: CompletedProcess = cmd("update", "0.2.0")
    assert proc.returncode != 0


@pytest.mark.parametrize(
    "env_path",
    **get_env_dirs("return-0"),
)
def test_valid_env(cmd, env_path: Path, tmp_path: Path):
    """Run test caese on env having valid files."""
    shutil.copytree(env_path / "before", tmp_path, dirs_exist_ok=True)
    proc: CompletedProcess = cmd("update", "0.2.0")
    assert proc.returncode == 0
    diff = run(["diff", "--recursive", str(tmp_path), str(env_path / "after")])
    assert diff.returncode == 0
