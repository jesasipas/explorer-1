"""Chargement et validation de la configuration réseau.

La configuration vit dans ``config/network.yaml``, fichier local jamais versionné.
Son chemin peut être surchargé par la variable d'environnement ``EXPLORER_NETWORK_CONFIG``.
"""

from __future__ import annotations

import os
from dataclasses import dataclass
from pathlib import Path
from typing import Any

import yaml

ENV_VAR = "EXPLORER_NETWORK_CONFIG"
DEFAULT_PATH = Path("config/network.yaml")

# Plage de ROS_DOMAIN_ID sûre sur Linux (pas de collision avec les ports éphémères).
DOMAIN_ID_MIN = 0
DOMAIN_ID_MAX = 101

SUPPORTED_RMW = frozenset({"rmw_cyclonedds_cpp", "rmw_fastrtps_cpp", "rmw_zenoh_cpp"})


class ConfigError(ValueError):
    """Configuration absente ou invalide."""


@dataclass(frozen=True)
class NetworkConfig:
    """Paramètres réseau partagés par le robot et le poste opérateur."""

    robot_host: str
    ros_domain_id: int
    rmw_implementation: str


def resolve_path(path: Path | None = None) -> Path:
    """Retourne le chemin explicite, sinon celui de l'environnement, sinon le chemin par défaut."""
    if path is not None:
        return path
    env_value = os.environ.get(ENV_VAR)
    return Path(env_value) if env_value else DEFAULT_PATH


def parse_network_config(data: Any) -> NetworkConfig:
    """Valide un dictionnaire déjà chargé et construit un :class:`NetworkConfig`."""
    if not isinstance(data, dict):
        raise ConfigError("la configuration doit être un dictionnaire YAML")

    robot = data.get("robot")
    ros = data.get("ros")
    if not isinstance(robot, dict) or not isinstance(ros, dict):
        raise ConfigError("les sections 'robot' et 'ros' sont obligatoires")

    host = robot.get("host")
    if not isinstance(host, str) or not host.strip():
        raise ConfigError("'robot.host' doit être une chaîne non vide")

    domain_id = ros.get("domain_id")
    # bool est un sous-type de int en Python : on l'exclut explicitement.
    if isinstance(domain_id, bool) or not isinstance(domain_id, int):
        raise ConfigError("'ros.domain_id' doit être un entier")
    if not DOMAIN_ID_MIN <= domain_id <= DOMAIN_ID_MAX:
        raise ConfigError(f"'ros.domain_id' doit être entre {DOMAIN_ID_MIN} et {DOMAIN_ID_MAX}")

    rmw = ros.get("rmw_implementation", "rmw_cyclonedds_cpp")
    if rmw not in SUPPORTED_RMW:
        raise ConfigError(f"'ros.rmw_implementation' doit être parmi {sorted(SUPPORTED_RMW)}")

    return NetworkConfig(robot_host=host.strip(), ros_domain_id=domain_id, rmw_implementation=rmw)


def load_network_config(path: Path | None = None) -> NetworkConfig:
    """Lit et valide le fichier de configuration réseau."""
    config_path = resolve_path(path)
    if not config_path.is_file():
        raise ConfigError(
            f"fichier introuvable : {config_path} "
            "(copier config/network.example.yaml en config/network.yaml)"
        )
    with config_path.open(encoding="utf-8") as handle:
        data = yaml.safe_load(handle)
    return parse_network_config(data)
