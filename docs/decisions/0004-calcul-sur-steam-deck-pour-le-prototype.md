# 0004 — Calcul sur le Steam Deck pour le prototype

- **Statut** : accepté
- **Date** : 2026-10-29

## Contexte
L'objectif final exige un robot autonome même sans liaison avec l'opérateur, donc des calculs
embarqués. Mais au stade du prototype, déboguer et itérer sur le Raspberry Pi 5 est lent, et
ses capacités pour SLAM + navigation + vision sont inconnues.

## Décision
Choix **volontaire** et temporaire : SLAM, navigation et exploration tournent sur le Steam Deck.
Deux garde-fous :
1. Les fonctions de sécurité (arrêt si plus de commande, arrêt sur obstacle proche) tournent
   **toujours sur le robot**, dès le jalon M2.
2. Tous les nœuds sont portables : aucune hypothèse sur la machine qui les exécute.

**Condition de sortie** : migration sur le robot avant le jalon M5 (perte de liaison), guidée
par les mesures du jalon M1.

## Conséquences
- Itérations rapides et débogage confortable.
- Le Wi-Fi est dans la boucle : latence et coupures doivent être mesurées et tolérées.
- Synchronisation des horloges obligatoire (chrony).
- Tant que la migration n'est pas faite, une coupure arrête la mission (le robot s'immobilise).
