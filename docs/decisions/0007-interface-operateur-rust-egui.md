# 0007 — Interface opérateur en Rust/egui

- **Statut** : accepté
- **Date** : 2026-11-19

## Contexte
L'[ADR 0006](0006-pyside6.md) retenait PySide6 pour l'interface opérateur. En pratique, cette
interface n'a besoin d'aucune brique ROS 2 : elle consomme un flux vidéo, une télémétrie
(batterie, réseau, lidar) et envoie des consignes, via une frontière déjà posée par
l'[ADR 0003](0003-frontiere-materielle.md). Elle tourne sur le Steam Deck, dont le mode jeu
privilégie les applications natives packagées simplement (un binaire), et la boucle de rendu
(vidéo + mini-carte lidar animée) bénéficie d'un contrôle fin des performances et des threads.

## Décision
Réécrire l'interface opérateur en **Rust**, avec **egui/eframe** pour le rendu immédiat. Elle
reste un projet Cargo indépendant (`ui/`), sans aucune dépendance à ROS 2 ou `rclpy` : elle ne
connaît qu'un contrat de données (télémétrie, consignes, vidéo) alimenté par un adaptateur
séparé.

Cette décision ne change pas l'[ADR 0005](0005-python-par-defaut.md) : Python reste la règle
par défaut pour les nœuds ROS 2. L'interface opérateur est une exception assumée, hors du
périmètre ROS 2, au même titre qu'elle l'était déjà sous PySide6.

## Conséquences
- Un seul binaire à distribuer sur le Steam Deck, sans environnement Python/Qt à déployer.
- La maquette de référence ([`docs/maquette/`](../maquette/README.md)) reste valable : seule la
  technologie de rendu change, pas la disposition, les états ni les seuils.
- Règle héritée du prototype Qt, transposée : le thread d'interface (boucle egui) ne doit
  jamais être bloqué par les entrées/sorties réseau ou manette ; aucun état d'interface n'est
  partagé par verrou entre threads, la communication se fait par canaux (`std::sync::mpsc`)
  et `egui::Context::request_repaint()` (voir `docs/troubleshooting.md`, section Interface).
- L'écosystème de tests Rust (tests unitaires, `egui_kittest` pour le rendu) remplace
  pytest/mypy pour ce sous-projet ; la CI gagne un job Rust dédié.
- L'[ADR 0006](0006-pyside6.md) est remplacé par le présent ADR.
