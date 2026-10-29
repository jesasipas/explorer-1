"""Tests du chargement de la configuration réseau."""

from __future__ import annotations

from pathlib import Path
from typing import Any

import pytest

from explorer_operator.config import (
    ENV_VAR,
    ConfigError,
    load_network_config,
    parse_network_config,
)

REPO_ROOT = Path(__file__).resolve().parents[4]
EXAMPLE = REPO_ROOT / "config" / "network.example.yaml"


def valid() -> dict[str, Any]:
    return {
        "robot": {"host": "explorer-1.local"},
        "ros": {"domain_id": 42, "rmw_implementation": "rmw_zenoh_cpp"},
    }


def test_example_file_is_valid() -> None:
    """Le modèle versionné doit toujours rester chargeable."""
    config = load_network_config(EXAMPLE)
    assert config.robot_host == "explorer-1.local"
    assert 0 <= config.ros_domain_id <= 101


def test_parse_valid_config() -> None:
    config = parse_network_config(valid())
    assert config.ros_domain_id == 42
    assert config.rmw_implementation == "rmw_zenoh_cpp"


def test_rmw_defaults_to_cyclonedds() -> None:
    data = valid()
    del data["ros"]["rmw_implementation"]
    assert parse_network_config(data).rmw_implementation == "rmw_cyclonedds_cpp"


@pytest.mark.parametrize(
    ("section", "key", "value"),
    [
        ("robot", "host", ""),
        ("robot", "host", 12),
        ("ros", "domain_id", 102),
        ("ros", "domain_id", -1),
        ("ros", "domain_id", True),
        ("ros", "domain_id", "42"),
        ("ros", "rmw_implementation", "rmw_inconnu"),
    ],
)
def test_invalid_values_are_rejected(section: str, key: str, value: object) -> None:
    data = valid()
    data[section][key] = value
    with pytest.raises(ConfigError):
        parse_network_config(data)


@pytest.mark.parametrize("data", [None, [], {"robot": {}}, {"ros": {"domain_id": 1}}])
def test_missing_sections_are_rejected(data: object) -> None:
    with pytest.raises(ConfigError):
        parse_network_config(data)


def test_missing_file_gives_helpful_error(tmp_path: Path) -> None:
    with pytest.raises(ConfigError, match=r"network\.example\.yaml"):
        load_network_config(tmp_path / "absent.yaml")


def test_env_var_overrides_default_path(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    custom = tmp_path / "custom.yaml"
    custom.write_text("robot: {host: robot-test}\nros: {domain_id: 7}\n", encoding="utf-8")
    monkeypatch.setenv(ENV_VAR, str(custom))
    assert load_network_config().robot_host == "robot-test"
