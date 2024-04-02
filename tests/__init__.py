# noqa: D104
from pathlib import Path

root = Path(__file__).parent


def get_env_dirs(name: str):
    """Return structure of folder-path as test pattern.

    .. note:: This is for parametrize.
    """
    paths = [p for p in (root / name).glob("*") if p.is_dir()]
    return {"argvalues": paths, "ids": [f"{p.parent.name}/{p.name}" for p in paths]}
