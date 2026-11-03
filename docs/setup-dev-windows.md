# Poste de développement sous Windows (VS Code + Dev Container)

VS Code tourne sous Windows, mais le code s'exécute dans un **conteneur Linux** (Ubuntu 22.04 +
ROS 2 Humble) défini dans `.devcontainer/`. Résultat : le même environnement pour tout le monde,
sans rien installer de ROS à la main sous Windows.

```
VS Code (Windows)  ──►  extension Dev Containers  ──►  Docker Desktop (WSL 2)  ──►  conteneur ROS 2 Humble
                                                                                    └─ dépôt explorer-1
```

Durée : environ 30 minutes, dont 10 à 15 minutes de téléchargement au premier lancement.

## 1. Installer WSL 2

Dans **PowerShell en administrateur** :

```powershell
wsl --install
```

Redémarrer le PC. Au redémarrage, une fenêtre Ubuntu demande un nom d'utilisateur et un mot de
passe Linux : les choisir (ils ne servent qu'à cet Ubuntu).

Vérifier : `wsl --status` doit indiquer la version par défaut **2**.

> Si la virtualisation est désactivée, l'installation échoue : l'activer dans le BIOS/UEFI
> (option « Intel VT-x » ou « AMD-V / SVM »).

## 2. Installer Docker Desktop

1. Télécharger et installer Docker Desktop pour Windows depuis le site officiel de Docker.
2. À l'installation, laisser cochée l'option **« Use WSL 2 instead of Hyper-V »**.
3. Lancer Docker Desktop et attendre que la baleine indique « Engine running ».
4. Dans *Settings → Resources → WSL integration*, activer l'intégration avec la distribution Ubuntu.

Vérifier dans PowerShell : `docker run --rm hello-world` affiche « Hello from Docker! ».

## 3. Installer Git pour Windows

Télécharger et installer Git pour Windows (options par défaut). Il fournit le gestionnaire
d'identifiants Git, que le conteneur réutilise pour pousser sur GitHub.

Puis, dans PowerShell, une seule fois :

```powershell
git config --global user.name "Thomas Blondel"
git config --global user.email "thomas.blondel@viacesi.fr"
git config --global core.autocrlf false
```

`core.autocrlf false` évite que Windows convertisse les fins de ligne (le dépôt les fixe déjà
avec `.gitattributes`).

## 4. Préparer VS Code

Installer l'extension **Dev Containers** (`ms-vscode-remote.remote-containers`).
C'est la seule extension nécessaire côté Windows : les autres (Python, Ruff, Mypy, ROS 2,
YAML, XML…) s'installent automatiquement **dans** le conteneur.

## 5. Ouvrir le projet dans le conteneur

**Méthode recommandée : cloner directement dans un volume Docker** (rapide, pas de problème
de fins de ligne ni de lenteur des fichiers Windows).

1. Dans VS Code : `Ctrl+Maj+P` → **Dev Containers: Clone Repository in Container Volume…**
2. Coller `https://github.com/jesasipas/explorer-1`
3. Attendre la construction du conteneur (10 à 15 minutes la première fois, quelques secondes
   ensuite). Le script `.devcontainer/post-create.sh` termine l'installation : configuration
   réseau locale, hooks pre-commit, dépendances ROS 2, première construction du workspace.

> À éviter : cloner dans `C:\Users\...` puis « Reopen in Container ». Ça fonctionne, mais les
> accès aux fichiers Windows depuis le conteneur sont très lents.

## 6. Vérifier que tout marche

Ouvrir un terminal dans VS Code (`Ctrl+ù`), il s'ouvre **dans le conteneur** :

```bash
ros2 --help | head -3          # ROS 2 Humble disponible
pytest                         # 30 tests au vert
```

Puis `Ctrl+Maj+P` → **Tasks: Run Task** :

| Tâche | Rôle |
|---|---|
| colcon : construire (`Ctrl+Maj+B`) | Construit le workspace ROS 2 |
| Vérifier (lint + typage + tests) | Les mêmes contrôles que la CI GitHub |
| Formater le code | Corrige le style automatiquement |
| ROS 2 : lister les topics | Vérifie la communication ROS 2 |

Le code Python est formaté à chaque sauvegarde, les erreurs de typage apparaissent directement
dans l'éditeur, et les tests sont listés dans l'onglet **Testing** (icône fiole).

## Limites connues

- **Parler au robot depuis le conteneur.** Sous Windows, Docker Desktop isole le réseau du
  conteneur : la découverte ROS 2 avec le robot ne fonctionne pas telle quelle. Ce poste sert
  à développer et tester ; les essais avec le robot se font depuis le Steam Deck
  (`docs/setup-steamdeck.md`). Une solution sera étudiée au jalon M1 (mode réseau « host » de
  Docker Desktop, ou Zenoh avec un routeur).
- **Interfaces graphiques (RViz, interface PySide6).** Pas d'affichage graphique depuis le
  conteneur pour l'instant ; à ajouter quand l'interface arrivera (jalon M2).

## Dépannage

| Symptôme | Piste |
|---|---|
| « Docker is not running » | Lancer Docker Desktop et attendre « Engine running » |
| La construction échoue au téléchargement | Vérifier la connexion internet, puis `Ctrl+Maj+P` → **Dev Containers: Rebuild Container** |
| `$'\r': command not found` dans un script | Fichier passé en fins de ligne Windows : vérifier `core.autocrlf false` et recloner |
| Une extension manque dans le conteneur | `Ctrl+Maj+P` → **Dev Containers: Rebuild Container** |
| Après modification de `.devcontainer/` | **Dev Containers: Rebuild Container** pour appliquer |
