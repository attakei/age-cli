"""Test case for ``age update``."""

import textwrap
from pathlib import Path


def test_not_configured_env(cmd):
    """'update' command requires configuration file.

    If age does not find configuration file, it should display message.
    """
    proc = cmd("update", "0.2.0")
    assert proc.returncode == 1
    assert "not exists." in proc.stderr


def test_empty_conf(cmd, tmp_path: Path):  # noqa: D103
    (tmp_path / ".age.toml").touch()
    proc = cmd("update", "0.2.0")
    assert proc.returncode == 1


def test_invalid_conf(cmd, tmp_path: Path):  # noqa: D103
    conf_text = textwrap.dedent(
        """\
    current_version = "0.0.0"
    """
    )
    (tmp_path / ".age.toml").write_text(conf_text)

    proc = cmd("update", "0.2.0")
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

    proc = cmd("update", "-1")
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

    proc = cmd("update", "0.2.0")
    assert proc.returncode == 0
    assert data_txt.read_text() == "0.2.0"
    assert 'current_version = "0.2.0"' in (tmp_path / ".age.toml").read_text()


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

    v{{new_version}}
    ------
    """
    '''
    )
    (tmp_path / ".age.toml").write_text(conf_text)
    data_txt = tmp_path / "data.txt"
    before = textwrap.dedent("""\
    ====
    Data
    ====

    Target
    ======
    """)
    after = textwrap.dedent("""\
    ====
    Data
    ====

    Target
    ======

    v0.2.0
    ------
    """)
    data_txt.write_text(before)

    proc = cmd("update", "0.2.0")
    print(data_txt.read_text())
    assert proc.returncode == 0
    assert len(data_txt.read_text().split("\n")) == 10
    assert data_txt.read_text() == after
