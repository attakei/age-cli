"""Common test contexts."""

from pathlib import Path
from subprocess import PIPE, run

import pytest

project_root = Path(__file__).parent.parent


@pytest.fixture(scope="session", autouse=True)
def build_test_target():
    """Generate age binary for testing."""
    run(["cargo", "build"], stdout=PIPE, stderr=PIPE, cwd=project_root)


@pytest.fixture
def root() -> Path:
    """Path-object of project root."""
    return project_root


@pytest.fixture
def target_bin(root: Path) -> str:
    """Full-path of binary generated by ``cargo build``."""
    bin_path = root / "target" / "debug" / "age"
    return str(bin_path.resolve())