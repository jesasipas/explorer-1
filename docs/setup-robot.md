# Mise en route du robot (MentorPi M1)

> Les commandes ci-dessous ont été validées sur le prototype précédent (février 2026).
> Elles seront revérifiées au jalon M1.

## 1. Démarrage

Appuyer sur le bouton d'alimentation de la carte RRC Lite. Le démarrage prend environ une minute.

| Signal | Signification |
|---|---|
| LED2 clignote lentement | Démarrage en cours |
| LED1 bleue | ROS 2 prêt |
| Bip du buzzer | Configuration terminée |

Pas de bip : vérifier les câbles, en particulier l'IMU.

## 2. Réseau

Le robot propose deux modes :

- **Point d'accès (défaut)** : le robot crée son propre réseau Wi-Fi. Pratique pour un premier
  contact, sans accès internet.
- **Client (STA)** : le robot rejoint un routeur existant. Recommandé pour le développement.

Le mode se règle dans l'outil réseau HiWonder sur le robot (`~/hiwonder-toolbox/wifi_conf.py`),
puis `sudo reboot`. **Ne jamais recopier le nom du réseau ni le mot de passe dans ce dépôt.**

Reporter ensuite le nom d'hôte ou l'adresse du robot dans `config/network.yaml`.

## 3. Sécuriser l'accès — à faire en premier

L'image HiWonder est livrée avec des identifiants par défaut publics. Avant toute connexion à
un réseau partagé :

```bash
passwd                      # changer le mot de passe de l'utilisateur
# puis, idéalement : authentification SSH par clé et désactivation du mot de passe
```

Changer aussi le mot de passe du point d'accès Wi-Fi du robot.

## 4. ROS 2 sur le robot

La pile HiWonder (ROS 2 Humble) tourne dans un conteneur Docker lancé au démarrage.

```bash
docker ps                                   # repérer le conteneur
docker exec -it -u ubuntu <conteneur> bash
source ~/ros2_ws/.typerc && source ~/ros2_ws/install/setup.bash
ros2 node list
```

Points de vigilance relevés sur le prototype :

- Le conteneur doit utiliser le **réseau de l'hôte** (`network_mode: host`), sinon ses topics
  sont invisibles depuis l'extérieur. Vérifier avec `docker inspect <conteneur> | grep NetworkMode`.
- Un service systemd HiWonder peut lancer les nœuds en double ; vérifier avec
  `systemctl status mentorpi-bringup.service`.
- Pour que le Steam Deck voie les topics, le conteneur doit exporter le même `ROS_DOMAIN_ID`
  et le même `RMW_IMPLEMENTATION` que `config/network.yaml`. [TODO M1 : documenter où les régler]

## 5. Synchronisation des horloges

```bash
sudo apt install chrony   # sur le robot ; configurer le Steam Deck comme serveur ou une source commune
chronyc tracking          # écart attendu : quelques millisecondes
```

[TODO M1 : configuration chrony validée.]

## 6. Vérifications rapides

```bash
ros2 topic list
ros2 topic hz /scan_raw            # lidar
ros2 topic echo /imu_corrected --once
```

Test moteur, **roues levées** :

```bash
ros2 topic pub --once /controller/cmd_vel geometry_msgs/msg/Twist "{linear: {x: 0.1}}"
ros2 topic pub --once /controller/cmd_vel geometry_msgs/msg/Twist "{}"
```
