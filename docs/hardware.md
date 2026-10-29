# Matériel

Plateforme : **HiWonder MentorPi M1**, châssis à roues Mecanum.
Documentation officielle : <https://docs.hiwonder.com/projects/MentorPi/en/latest/>

## Vue d'ensemble

```
[Raspberry Pi 5]  ← calcul principal (ROS 2, pilotes)
      │ série
[Carte RRC Lite]  ← sous-contrôleur temps réel
      ├── 4 moteurs DC à encodeurs → roues Mecanum
      ├── IMU (accéléromètre + gyroscope)
      ├── 2 servos PWM (support caméra pan/tilt)
      └── buzzer, 2 LED, 2 boutons (KEY1 auto-test, KEY2 extinction)

[Lidar 360°]  ── USB → Raspberry Pi 5
[Caméra]      ── USB → Raspberry Pi 5
```

## Composants

| Composant | Détails connus | À confirmer |
|---|---|---|
| Raspberry Pi 5 | Quad-core Cortex-A76 2,4 GHz, Wi-Fi bi-bande | RAM (4 ou 8 Go) |
| Carte RRC Lite | Pilote moteurs en boucle fermée, IMU, servos ; pilotée par `ros_robot_controller` | — |
| Lidar | TOF 360°, environ 10 Hz | Modèle : la doc mentionne un STL-19P, le pilote a annoncé un MS200 |
| Caméra | Montée sur 2 servos (pan/tilt) | Monoculaire (`usb_cam`) ou profondeur (`ascamera`) ? Indispensable pour la carte sémantique (M6) |
| Batterie | LiPo 7,4 V 2200 mAh (2S) | — |

## Roues Mecanum

Le robot est **holonome** : il peut avancer, glisser latéralement et tourner indépendamment.
La consigne `cmd_vel` utilise donc `linear.x` (avant), `linear.y` (latéral) et `angular.z`
(rotation). La conversion en vitesses de roues est faite par la pile HiWonder.

## Consignes de sécurité matérielle

- Charger complètement la batterie avant la première utilisation.
- Plage de tension LiPo 2S : environ 6,4 V (vide) à 8,4 V (pleine). Ne pas descendre sous le seuil bas.
- **Ne jamais couper l'alimentation brutalement** (risque de corruption de la carte microSD) :
  éteindre avec `sudo shutdown -h now` ou un appui de 2 s sur KEY2.
- Premiers essais de motorisation **roues levées**.
