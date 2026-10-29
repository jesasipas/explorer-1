# Architecture

## Principes

1. **S'appuyer sur l'écosystème ROS 2.** Cartographie (slam_toolbox), navigation (Nav2) et
   pilotes existent et sont éprouvés. Le code d'explorer-1 se concentre sur l'intégration, la
   mission et la perception.
2. **Réflexes sur le robot, réflexion où l'on veut.** Tout ce qui protège le robot (arrêt si
   plus de commande, arrêt sur obstacle proche) tourne toujours sur le robot. Le reste est écrit
   en nœuds portables qui peuvent tourner sur le Steam Deck ou sur le robot.
3. **Frontière matérielle explicite.** Le code ne connaît que des topics génériques ; la
   correspondance avec les topics du robot se fait uniquement dans `explorer_bringup`
   ([ADR 0003](decisions/0003-frontiere-materielle.md)).
4. **Aucune donnée sensible dans le dépôt.** Réseau et identifiants vivent dans
   `config/network.yaml`, non versionné.

## Répartition des calculs (prototype)

| Fonction | Machine | Raison |
|---|---|---|
| Moteurs, encodeurs, IMU | Carte RRC Lite | Temps réel matériel |
| Pilotes lidar / caméra / contrôleur | Raspberry Pi 5 (pile HiWonder) | Fournis par le fabricant |
| Nœud de sécurité (M2) | Raspberry Pi 5 | Doit fonctionner même sans Wi-Fi |
| SLAM, Nav2, exploration | Steam Deck | Choix volontaire pour le prototype ([ADR 0004](decisions/0004-calcul-sur-steam-deck-pour-le-prototype.md)) |
| Interface opérateur | Steam Deck | — |

Avant le jalon M5 (perte de liaison), SLAM, Nav2 et le gestionnaire de mission migrent sur le
robot, sous réserve des mesures du jalon M1.

## Topics

Noms génériques utilisés par explorer-1, et correspondance sur le MentorPi M1 :

| Générique | MentorPi M1 (pile HiWonder) | Type |
|---|---|---|
| `cmd_vel` | `/controller/cmd_vel` | `geometry_msgs/Twist` |
| `scan` | `/scan` (filtré, via bringup complet) ou `/scan_raw` | `sensor_msgs/LaserScan` |
| `odom` | `/odom` (via bringup complet) | `nav_msgs/Odometry` |
| `imu` | `/imu_corrected` | `sensor_msgs/Imu` |
| `battery` | `/ros_robot_controller/battery` (mV) | `std_msgs/UInt16` |
| `image` | dépend de la caméra — [TODO M1] | `sensor_msgs/Image` |

Ces noms ont été relevés sur le prototype (février–mars 2026) et sont à revérifier au jalon M1.

## Communication réseau

- Robot et Steam Deck partagent le même `ROS_DOMAIN_ID` et la même couche de communication
  (`RMW_IMPLEMENTATION`), définis dans `config/network.yaml`.
- Le choix entre Cyclone DDS (pairs unicast) et Zenoh est tranché par les mesures du jalon M1
  ([ADR 0002](decisions/0002-couche-de-communication.md)).
- Horloges synchronisées par chrony (indispensable pour les repères `tf` quand le calcul est
  déporté).
- Les images circulent compressées.
