# 0002 — Couche de communication : Cyclone DDS ou Zenoh

- **Statut** : proposé (à trancher au jalon M1)
- **Date** : 2026-10-29

## Contexte
Robot et Steam Deck communiquent en ROS 2 natif, sans passerelle WebSocket (le prototype
utilisait rosbridge). La couche DDS par défaut découvre ses pairs par multicast, ce que le
Wi-Fi et les box gèrent mal ; les gros messages peuvent saturer la liaison.

## Options
1. **Cyclone DDS** avec une liste de pairs explicite (pas de multicast). Simple, bien connu.
2. **Zenoh** (`rmw_zenoh_cpp`, disponible en paquet binaire pour Humble). Conçu pour les
   réseaux peu fiables et le passage à l'échelle (utile pour l'essaim). Impose que **tous**
   les nœuds, y compris la pile HiWonder, utilisent la même couche, et nécessite un routeur Zenoh.

## Décision
Valeur par défaut provisoire : Cyclone DDS. Décision définitive selon les mesures
(`docs/benchmarks/`) : fréquence reçue, latence, pertes en mouvement, facilité de configuration
de la pile HiWonder.

## Conséquences
La couche est un paramètre (`config/network.yaml`), pas un choix codé en dur : en changer ne
demande aucune modification du code.
