# 0003 — Frontière matérielle par remappage des topics

- **Statut** : accepté
- **Date** : 2026-10-29

## Contexte
La pile du fabricant utilise ses propres noms de topics (`/controller/cmd_vel`, `/scan_raw`…).
Si ces noms se dispersent dans le code, explorer-1 devient prisonnier de ce matériel, et la
simulation comme l'essaim deviennent pénibles.

## Décision
Les nœuds d'explorer-1 n'utilisent que des noms génériques (`cmd_vel`, `scan`, `odom`, `imu`,
`image`). La correspondance avec un robot donné se fait **uniquement** dans les fichiers de
lancement du paquet `explorer_bringup`, par remappage.

## Conséquences
- Passer du robot réel à la simulation, ou à un autre robot, ne touche que `explorer_bringup`.
- Les espaces de noms par robot (essaim, M7) s'ajoutent au même endroit.
- La table de correspondance est tenue à jour dans `docs/architecture.md`.
