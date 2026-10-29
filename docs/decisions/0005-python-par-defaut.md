# 0005 — Python par défaut, C++ selon une règle mesurable

- **Statut** : accepté
- **Date** : 2026-10-29

## Contexte
Les briques lourdes (slam_toolbox, Nav2, pilotes) sont déjà en C++. Le code propre à
explorer-1 est surtout de l'orchestration : mission, téléopération, interface, perception de
haut niveau. Python domine la vision par ordinateur et correspond au profil de l'auteur.
Mais `rclpy` est nettement plus lent que `rclcpp`, et le verrou global de Python limite le
parallélisme.

## Décision
Python 3.10 par défaut, typé (mypy strict), testé (pytest). Un nœud est écrit ou réécrit en
C++ si :
- les mesures montrent qu'il dépasse son budget CPU ou de latence (seuils fixés au jalon M1) ;
- ou il doit s'intégrer comme extension de Nav2 (contrôleur, planificateur, nœud d'arbre de
  comportement), ce que seul le C++ permet ;
- ou il doit échanger des images sans copie (nœuds composables).

## Conséquences
- Développement rapide, accès direct à l'écosystème vision (OpenCV, ONNX Runtime…).
- La logique est isolée de ROS (ex. `explorer_operator.teleop`) : testable sans robot, et
  portable si une réécriture s'impose.
- Les interfaces entre nœuds (messages) restent stables : réécrire un nœud ne casse rien autour.
