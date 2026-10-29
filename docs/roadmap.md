# Feuille de route

Chaque jalon correspond à un *milestone* GitHub ; les issues sont créées par
`scripts/github/bootstrap.sh`.

## M0 — Fondations ✅ (en cours de revue)
Dépôt propre, outillage qualité, CI, documentation migrée depuis Notion, configuration réseau.

## M1 — Environnement et mesures
- ROS 2 Humble dans distrobox sur le Steam Deck
- Liaison robot ↔ Steam Deck, synchronisation des horloges (chrony)
- Mesures : latence et pertes sur `scan`, débit vidéo, CPU de slam_toolbox sur le Deck et sur le Pi 5
- Comparaison Cyclone DDS / Zenoh → décision ADR 0002
- Confirmation du modèle de lidar et du type de caméra

## M1.5 — Simulation
- Modèle de robot à roues Mecanum dans Gazebo
- Même lancement en simulation et sur le robot (grâce à la frontière matérielle)
- Démo vidéo pour le README

## M2 — Téléopération sûre
- Nœud de sécurité **sur le robot** : arrêt si plus de commande (environ 300 ms), arrêt sur obstacle proche
- Vérification du mapping manette du Steam Deck
- Interface PySide6 : vidéo, télémétrie (batterie, réseau), arrêt d'urgence

## M3 — Cartographie 2D
- slam_toolbox configuré pour le robot
- Affichage de la carte dans l'interface
- Sauvegarde et rechargement des cartes

## M4 — Exploration autonome
- Nav2 configuré pour un robot holonome
- Exploration automatique des frontières
- Retour au point de départ

## M5 — Résilience
- Migration de SLAM, Nav2 et de la mission sur le Raspberry Pi 5
- Gestionnaire de mission (machine à états)
- Comportement défini en cas de perte de liaison ; enregistrement local des données

## M6 — Carte sémantique
- Choix matériel pour la détection embarquée (accélérateur dédié ?)
- Détection d'objets sur le flux caméra
- Projection des détections sur la carte (profondeur + calibration caméra ↔ lidar)
- Étiquettes nommées dans l'interface

## M7 — Essaim
- Espaces de noms par robot
- Fusion des cartes
- Répartition automatique des zones à explorer

## Plus tard
- Migration vers ROS 2 Jazzy (Humble arrive en fin de support le 31 mai 2027)
