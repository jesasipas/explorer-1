# 0001 — ROS 2 Humble, dans distrobox sur le Steam Deck

- **Statut** : accepté
- **Date** : 2026-10-29

## Contexte
Le robot est livré avec ROS 2 Humble (pile HiWonder, dans Docker). Faire communiquer deux
versions différentes de ROS 2 n'est pas officiellement supporté. SteamOS a un système en
lecture seule, peu adapté à une installation native de ROS 2.

## Décision
Utiliser ROS 2 Humble partout. Sur le Steam Deck, l'installer dans un conteneur Ubuntu 22.04
géré par distrobox (`docker/distrobox.ini`), qui partage réseau, affichage et dossier personnel.

## Conséquences
- Même version des deux côtés : pas de surprise sur les types de messages.
- SteamOS reste intact ; l'environnement est reproductible sur n'importe quel PC Linux.
- Humble arrive en fin de support le 31 mai 2027 : une migration vers Jazzy (support jusqu'en
  2029) est inscrite à la feuille de route. Le code évite toute dépendance propre à Humble.
