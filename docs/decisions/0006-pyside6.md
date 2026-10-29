# 0006 — PySide6 pour l'interface opérateur

- **Statut** : accepté
- **Date** : 2026-10-29

## Contexte
Le prototype utilisait PyQt6, distribué sous GPL v3 ou licence commerciale. Le dépôt est sous
licence MIT : associer les deux devient ambigu dès qu'on distribue l'application.

## Décision
Utiliser **PySide6** (Qt for Python, binding officiel de Qt), sous licence LGPL.

## Conséquences
- API quasi identique à PyQt6 : l'expérience acquise sur le prototype reste valable.
- Pas d'ambiguïté de licence pour un dépôt MIT.
- Règle héritée du prototype : aucun widget n'est modifié hors du thread Qt
  (voir `docs/troubleshooting.md`).
