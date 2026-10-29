# Décisions d'architecture (ADR)

Chaque décision technique structurante est consignée dans un fichier court :
contexte, décision, conséquences. Une décision n'est jamais effacée : si elle change,
un nouvel ADR la remplace et l'ancien passe au statut « remplacé ».

| N° | Décision | Statut |
|---|---|---|
| [0001](0001-ros2-humble-dans-distrobox.md) | ROS 2 Humble, dans distrobox sur le Steam Deck | Accepté |
| [0002](0002-couche-de-communication.md) | Couche de communication : Cyclone DDS ou Zenoh | Proposé (tranché en M1) |
| [0003](0003-frontiere-materielle.md) | Frontière matérielle par remappage des topics | Accepté |
| [0004](0004-calcul-sur-steam-deck-pour-le-prototype.md) | Calcul sur le Steam Deck pour le prototype | Accepté |
| [0005](0005-python-par-defaut.md) | Python par défaut, C++ selon une règle mesurable | Accepté |
| [0006](0006-pyside6.md) | PySide6 pour l'interface opérateur | Accepté |

Modèle : [`template.md`](template.md).
