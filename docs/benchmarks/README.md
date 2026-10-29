# Mesures de performance (jalon M1)

Objectif : décider, chiffres à l'appui, où tournent les calculs et quelle couche de
communication utiliser. Chaque campagne de mesure ajoute une ligne aux tableaux ci-dessous
et, si besoin, met à jour l'ADR concerné.

## Protocole

- Robot immobile puis en mouvement lent, dans la même pièce, même position du routeur.
- Durée de chaque mesure : 60 s minimum. Noter la version du dépôt (`git rev-parse --short HEAD`).
- Horloges synchronisées (chrony) avant toute mesure de latence.

| Mesure | Commande indicative | Seuil d'alerte (à affiner) |
|---|---|---|
| Fréquence du lidar reçue sur le Deck | `ros2 topic hz <scan>` | < 90 % de la fréquence émise |
| Latence du lidar | horodatage du message vs horloge locale | > 100 ms |
| Débit vidéo | `ros2 topic bw <image>` | saturation du lien |
| CPU de slam_toolbox | `top` / `pidstat` | > 70 % d'un cœur en continu |
| Pertes de messages en mouvement | compteur de séquence | > 2 % |

## Résultats

| Date | Version | Couche | Mesure | Valeur | Remarque |
|---|---|---|---|---|---|
| [TODO] | | | | | |
