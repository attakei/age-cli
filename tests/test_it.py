"""Test case for generals (none subcommands)."""


def test_vesion_display(cmd):  # noqa: D103
    proc = cmd("--version")
    assert proc.returncode == 0
    assert proc.stdout.startswith("age v")
