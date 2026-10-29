# Journal des problèmes résolus

Leçons tirées du prototype précédent (application de téléopération, février–mars 2026).
Le code de ce prototype n'a pas été repris, mais ces problèmes reviendront sous une forme
ou une autre : ils sont consignés ici avec leur cause et la parade retenue pour explorer-1.

## Robot

| Symptôme | Cause | Parade |
|---|---|---|
| Pas de bip au démarrage | Défaut matériel, souvent la connexion de l'IMU | Vérifier les câbles avant tout diagnostic logiciel |
| Topics du robot invisibles depuis l'extérieur | Conteneur Docker sans réseau de l'hôte | `network_mode: host` sur le conteneur ROS 2 |
| Nœuds lancés en double | Service systemd HiWonder en parallèle du conteneur | Vérifier `mentorpi-bringup.service`, n'en garder qu'un |
| Topics du lidar absents | Le lidar n'est pas toujours lancé au démarrage selon la configuration | Lancer le bringup complet ou le launch lidar ; vérifier avec `ros2 topic hz` |
| Noms de topics différents de la documentation (`/controller/cmd_vel`, `/scan_raw`) | Pile HiWonder spécifique | Frontière matérielle : remappage dans `explorer_bringup` uniquement ([ADR 0003](decisions/0003-frontiere-materielle.md)) |
| Risque de carte microSD corrompue | Coupure brutale de l'alimentation | Toujours `sudo shutdown -h now` ou KEY2 (2 s) |

## Données capteurs

| Symptôme | Cause | Parade |
|---|---|---|
| Plantage au traitement d'un scan lidar | Valeurs absentes, infinies ou NaN dans `ranges` | Filtrer systématiquement (`math.isfinite`, bornes `range_min` / `range_max`) avant tout calcul |

## Manette

| Symptôme | Cause | Parade |
|---|---|---|
| Boutons ou axes qui ne font pas ce qui est attendu | Les index varient selon la manette et le pilote ; la documentation de l'époque se contredisait | Index en paramètres ROS (`explorer_bringup/config/teleop.yaml`), vérifiés avec `ros2 topic echo /joy` |

## Interface (Qt)

| Symptôme | Cause | Parade |
|---|---|---|
| `signal only works in main thread`, mises à jour perdues, plantages | Des callbacks réseau (thread non Qt) touchaient directement l'interface | Règle pour explorer-1 : **aucun widget modifié hors du thread Qt**. Le thread ROS 2 transmet les données par signaux Qt en connexion différée (`Qt.QueuedConnection`) |
| Flux vidéo bloqué par la mise en page | Empilement de widgets mal adapté à une superposition vidéo + carte | Prévoir dès le départ une superposition dédiée (M2) |

## Réseau

| Symptôme | Cause | Parade |
|---|---|---|
| Robot injoignable après changement de box | Adresse codée en dur dans le code | Adresse uniquement dans `config/network.yaml` (non versionné) ; nom d'hôte plutôt qu'adresse IP |
