# Environnement ROS 2 sur le Steam Deck

SteamOS a un système en lecture seule : ROS 2 Humble est donc installé dans un conteneur
Ubuntu 22.04 géré par **distrobox** ([ADR 0001](decisions/0001-ros2-humble-dans-distrobox.md)).
Le conteneur partage le réseau, l'affichage et le dossier personnel du Steam Deck.

> Procédure à valider au jalon M1.

## 1. Vérifier distrobox

En mode Bureau, dans un terminal :

```bash
distrobox --version
```

S'il est absent, suivre la documentation d'installation de distrobox pour SteamOS
(installation dans le dossier personnel, sans toucher au système).

## 2. Créer le conteneur

Depuis la racine du dépôt :

```bash
distrobox assemble create --file docker/distrobox.ini
distrobox enter explorer-ros2
```

## 3. Construire le workspace

Dans le conteneur :

```bash
source /opt/ros/humble/setup.bash
cd ros2_ws
colcon build --symlink-install
source install/setup.bash
```

## 4. Variables réseau

Elles doivent correspondre à `config/network.yaml` et au robot :

```bash
export ROS_DOMAIN_ID=42
export RMW_IMPLEMENTATION=rmw_cyclonedds_cpp
```

[TODO M1 : script qui exporte ces variables depuis `config/network.yaml`.]

## 5. Vérifier la liaison avec le robot

```bash
ros2 topic list        # les topics du robot doivent apparaître
ros2 topic hz /scan_raw
```
