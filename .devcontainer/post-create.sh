#!/usr/bin/env bash
# Exécuté une seule fois, à la création du conteneur de développement.
# Idempotent : peut être relancé à la main sans risque.

set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> Configuration réseau locale"
if [ ! -f config/network.yaml ]; then
  cp config/network.example.yaml config/network.yaml
  echo "    config/network.yaml créé depuis le modèle (à adapter, il n'est pas versionné)"
fi

echo "==> Hooks pre-commit"
git config --global --add safe.directory "$PWD"
pre-commit install --install-hooks || echo "    (pre-commit sera installé au premier commit)"

echo "==> Dépendances ROS 2 du workspace"
# Les scripts setup.bash de ROS ne sont pas compatibles avec « set -u ».
set +u
source /opt/ros/humble/setup.bash
rosdep update --rosdistro humble
sudo apt-get update -qq
rosdep install --from-paths ros2_ws/src --ignore-src --rosdistro humble -y

echo "==> Première construction du workspace"
(cd ros2_ws && colcon build --symlink-install)

echo "==> Terminé. Ouvrir un nouveau terminal pour charger le workspace."
