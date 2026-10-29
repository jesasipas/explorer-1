"""Point d'entrée de l'application opérateur (Steam Deck).

Squelette : charge et affiche la configuration réseau. L'interface PySide6
(vidéo, carte, télémétrie) arrive au jalon M2 (voir docs/roadmap.md).
"""

from __future__ import annotations

import sys

from explorer_operator.config import ConfigError, load_network_config


def main() -> int:
    try:
        config = load_network_config()
    except ConfigError as error:
        print(f"[explorer-1] configuration invalide : {error}", file=sys.stderr)
        return 1

    print(
        f"[explorer-1] robot : {config.robot_host} | "
        f"ROS_DOMAIN_ID={config.ros_domain_id} | RMW={config.rmw_implementation}"
    )
    # [TODO jalon M2] lancer la fenêtre PySide6.
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
