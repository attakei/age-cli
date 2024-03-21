"""Test case for ``age update``."""

import textwrap
from pathlib import Path
from subprocess import PIPE, CompletedProcess, run
from typing import Callable, Tuple

import pytest


@pytest.fixture
def cmd(
    target_bin: str, tmp_path: Path
) -> Callable[[Tuple[..., str]], CompletedProcess]:
    """Shortcut to run command on subprocess."""

    def _cmd(*args) -> CompletedProcess:
        return run(
            [target_bin, "update"] + list(args),
            stdout=PIPE,
            stderr=PIPE,
            text=True,
            cwd=tmp_path,
        )

    return _cmd


def test_not_configured_env(cmd):
    """'update' command requires configuration file.

    If age does not find configuration file, it should display message.
    """
    proc = cmd("0.2.0")
    assert proc.returncode == 1
    assert "not exists." in proc.stderr


def test_empty_conf(cmd, tmp_path: Path):  # noqa: D103
    (tmp_path / ".age.toml").touch()
    proc = cmd("0.2.0")
    assert proc.returncode == 1


def test_invalid_conf(cmd, tmp_path: Path):  # noqa: D103
    conf_text = textwrap.dedent(
        """\
    current_version = "0.0.0"
    """
    )
    (tmp_path / ".age.toml").write_text(conf_text)

    proc = cmd("0.2.0")
    assert proc.returncode == 1


def test_invalid_args(cmd, tmp_path: Path):  # noqa: D103
    conf_text = textwrap.dedent(
        """\
    current_version = "0.0.0"

    [[files]]
    path = "data.txt"
    search = "{{current_version}}"
    replace = "{{new_version}}"
    """
    )
    (tmp_path / ".age.toml").write_text(conf_text)
    data_txt = tmp_path / "data.txt"
    data_txt.write_text("0.0.0")

    proc = cmd("-1")
    assert proc.returncode != 0


def test_valid_conf__single_line(cmd, tmp_path: Path):  # noqa: D103
    conf_text = textwrap.dedent(
        """\
    current_version = "0.0.0"

    [[files]]
    path = "data.txt"
    search = "{{current_version}}"
    replace = "{{new_version}}"
    """
    )
    (tmp_path / ".age.toml").write_text(conf_text)
    data_txt = tmp_path / "data.txt"
    data_txt.write_text("0.0.0")

    proc = cmd("0.2.0")
    assert proc.returncode == 0
    assert data_txt.read_text() == "0.2.0"


def test_valid_conf__multi_line(cmd, tmp_path: Path):  # noqa: D103
    conf_text = textwrap.dedent(
        '''\
    current_version = "0.0.0"

    [[files]]
    path = "data.txt"
    search = """
    Target
    ======
    """
    replace = """
    Target
    ======

    v {{new_version}}
    -----------------
    """
    '''
    )
    (tmp_path / ".age.toml").write_text(conf_text)
    data_txt = tmp_path / "data.txt"
    data_txt.write_text("Target\n======\n")

    proc = cmd("0.2.0")
    assert proc.returncode == 0
    assert len(data_txt.read_text().split("\n")) == 6
    assert "v 0.2.0" in data_txt.read_text()
