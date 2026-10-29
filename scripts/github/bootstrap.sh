#!/usr/bin/env bash
# Crée les étiquettes, jalons et issues initiales d'explorer-1 sur GitHub.
#
# Prérequis : GitHub CLI (`gh`) authentifié (`gh auth login`).
# Usage    : scripts/github/bootstrap.sh [owner/repo]
#
# Les étiquettes sont mises à jour si elles existent. Les jalons existants sont réutilisés.
# Les issues, elles, sont créées à chaque exécution : ne lancer ce script qu'une fois.

set -euo pipefail

REPO="${1:-$(gh repo view --json nameWithOwner -q .nameWithOwner)}"
echo "Dépôt cible : ${REPO}"

# ---------------------------------------------------------------- étiquettes
label() { gh label create "$1" --repo "$REPO" --color "$2" --description "$3" --force >/dev/null; }

label "type:feat"        "1D76DB" "Nouvelle fonctionnalité"
label "type:bug"         "D73A4A" "Comportement incorrect"
label "type:docs"        "0075CA" "Documentation"
label "type:chore"       "CFD3D7" "Outillage, maintenance"
label "type:test"        "BFD4F2" "Tests"
label "type:perf"        "FBCA04" "Mesures et performance"
label "area:operator"    "5319E7" "Steam Deck, interface, téléopération"
label "area:ros2"        "006B75" "Nœuds et lancements ROS 2"
label "area:perception"  "B60205" "Vision, carte sémantique"
label "area:infra"       "6E7781" "CI, environnement, réseau"
label "area:safety"      "E99695" "Sécurité du robot"
label "priority:high"    "B60205" "À traiter en priorité"
label "good first issue" "7057FF" "Bon point d'entrée"
echo "Étiquettes : OK"

# ---------------------------------------------------------------- jalons
milestone() {
  local title="$1" description="$2"
  if ! gh api "repos/${REPO}/milestones?state=all&per_page=100" -q '.[].title' | grep -qxF "$title"; then
    gh api "repos/${REPO}/milestones" -f title="$title" -f description="$description" >/dev/null
  fi
}

milestone "M0 Fondations"            "Dépôt propre, CI, documentation, configuration réseau"
milestone "M1 Environnement et mesures" "ROS 2 sur le Steam Deck, liaison, mesures de performance"
milestone "M1.5 Simulation"          "Robot Mecanum dans Gazebo"
milestone "M2 Téléopération sûre"    "Nœud de sécurité embarqué, manette, interface PySide6"
milestone "M3 Cartographie 2D"       "slam_toolbox, affichage et sauvegarde des cartes"
milestone "M4 Exploration autonome"  "Nav2, exploration des frontières, retour base"
milestone "M5 Résilience"            "Migration embarquée, mission, perte de liaison"
milestone "M6 Carte sémantique"      "Détection d'objets et étiquettes sur la carte"
milestone "M7 Essaim"                "Plusieurs robots coopérants"
echo "Jalons : OK"

# ---------------------------------------------------------------- issues
issue() {
  local milestone="$1" labels="$2" title="$3" body="$4"
  gh issue create --repo "$REPO" --milestone "$milestone" --label "$labels" \
    --title "$title" --body "$body" >/dev/null
  echo "  + ${title}"
}

echo "Issues :"

issue "M1 Environnement et mesures" "type:chore,area:infra,priority:high" \
  "Installer ROS 2 Humble dans distrobox sur le Steam Deck" \
  "Valider \`docker/distrobox.ini\` et \`docs/setup-steamdeck.md\` sur le Steam Deck.

- [ ] Conteneur créé et ROS 2 opérationnel
- [ ] Workspace construit avec colcon
- [ ] Versions figées dans distrobox.ini"

issue "M1 Environnement et mesures" "type:chore,area:infra,priority:high" \
  "Établir la liaison ROS 2 robot ↔ Steam Deck" \
  "- [ ] Même ROS_DOMAIN_ID et RMW des deux côtés (réglage côté conteneur HiWonder documenté)
- [ ] Conteneur du robot en réseau hôte
- [ ] Topics du robot visibles et reçus à pleine fréquence sur le Deck
- [ ] Script exportant les variables depuis config/network.yaml"

issue "M1 Environnement et mesures" "type:chore,area:infra" \
  "Synchroniser les horloges robot / Steam Deck (chrony)" \
  "Indispensable aux repères tf quand le calcul est déporté (ADR 0004).

- [ ] Configuration chrony documentée
- [ ] Écart mesuré < 5 ms"

issue "M1 Environnement et mesures" "type:perf,area:ros2,priority:high" \
  "Campagne de mesures : lidar, vidéo, CPU du SLAM" \
  "Suivre le protocole de \`docs/benchmarks/README.md\`.

- [ ] Fréquence et latence du lidar reçu sur le Deck
- [ ] Débit vidéo compressé
- [ ] CPU de slam_toolbox sur le Deck et sur le Pi 5
- [ ] Seuils CPU / latence fixés pour la règle Python → C++ (ADR 0005)"

issue "M1 Environnement et mesures" "type:perf,area:infra" \
  "Comparer Cyclone DDS et Zenoh et trancher l'ADR 0002" \
  "- [ ] Mesures en mouvement avec chaque couche
- [ ] Faisabilité côté pile HiWonder
- [ ] ADR 0002 passé au statut accepté"

issue "M1 Environnement et mesures" "type:docs,good first issue" \
  "Confirmer le modèle du lidar et le type de caméra" \
  "La documentation du prototype se contredit (STL-19P / MS200 ; usb_cam / ascamera).

- [ ] Modèles confirmés et reportés dans docs/hardware.md
- [ ] Table des topics de docs/architecture.md revérifiée"

issue "M1.5 Simulation" "type:feat,area:ros2" \
  "Simuler le robot à roues Mecanum dans Gazebo" \
  "- [ ] Modèle de robot holonome avec lidar et caméra
- [ ] Même launch qu'en réel grâce au remappage (ADR 0003)
- [ ] Courte vidéo de démo pour le README"

issue "M2 Téléopération sûre" "type:feat,area:safety,priority:high" \
  "Nœud de sécurité embarqué : watchdog et arrêt sur obstacle" \
  "Tourne sur le robot, indépendamment du Wi-Fi (ADR 0004).

- [ ] Consigne nulle si aucune commande depuis ~300 ms
- [ ] Arrêt si obstacle sous une distance seuil dans le sens de déplacement
- [ ] Tests unitaires de la logique, sans ROS"

issue "M2 Téléopération sûre" "type:feat,area:operator" \
  "Vérifier le mapping de la manette du Steam Deck" \
  "- [ ] Index des axes et boutons relevés avec \`ros2 topic echo /joy\`
- [ ] explorer_bringup/config/teleop.yaml mis à jour
- [ ] Gâchette de vitesse branchée sur speed_scale"

issue "M2 Téléopération sûre" "type:feat,area:operator" \
  "Interface opérateur PySide6 : vidéo, télémétrie, arrêt d'urgence" \
  "- [ ] Fenêtre adaptée au 1280×800
- [ ] Flux vidéo, batterie, qualité réseau
- [ ] Bouton d'arrêt d'urgence
- [ ] Aucun widget modifié hors du thread Qt"

issue "M3 Cartographie 2D" "type:feat,area:ros2" \
  "Configurer slam_toolbox et afficher la carte dans l'interface" \
  "- [ ] Paramètres slam_toolbox pour le robot
- [ ] Carte affichée en direct dans l'interface
- [ ] Sauvegarde et rechargement"

issue "M4 Exploration autonome" "type:feat,area:ros2" \
  "Navigation Nav2 et exploration des frontières" \
  "- [ ] Nav2 configuré pour un robot holonome
- [ ] Exploration automatique
- [ ] Retour au point de départ"

issue "M5 Résilience" "type:feat,area:ros2" \
  "Migrer l'autonomie sur le robot et gérer la perte de liaison" \
  "- [ ] SLAM, Nav2 et mission sur le Raspberry Pi 5
- [ ] Gestionnaire de mission (machine à états)
- [ ] Comportement défini en cas de coupure ; données enregistrées localement"

issue "M6 Carte sémantique" "type:feat,area:perception" \
  "Détecter des objets et les nommer sur la carte" \
  "- [ ] Choix matériel pour la détection embarquée
- [ ] Détection sur le flux caméra
- [ ] Projection sur la carte (profondeur + calibration caméra ↔ lidar)
- [ ] Étiquettes dans l'interface"

issue "M7 Essaim" "type:feat,area:ros2" \
  "Préparer le multi-robots : espaces de noms et fusion de cartes" \
  "- [ ] Espaces de noms par robot dans explorer_bringup
- [ ] Fusion de cartes en simulation"

echo "Terminé."
