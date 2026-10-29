"""Logique de téléopération : manette → consigne de vitesse.

Ce module est volontairement indépendant de ROS2 : il ne manipule que des nombres,
ce qui le rend testable sans robot ni ROS installé. Le nœud ROS2 (``teleop_node``)
se contente de lui passer les axes de la manette et de publier le résultat.

Conventions :
- les axes en entrée sont normalisés dans [-1, 1], déjà orientés
  (``forward > 0`` = avancer, ``strafe > 0`` = gauche, ``turn > 0`` = rotation à gauche),
  conformément à la convention ROS REP-103 ;
- les vitesses en sortie sont en m/s (linéaire) et rad/s (angulaire).
"""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class TeleopLimits:
    """Bornes appliquées à la consigne. Valeurs prudentes par défaut pour un robot d'intérieur."""

    max_linear: float = 0.3  # m/s
    max_angular: float = 1.0  # rad/s
    deadzone: float = 0.08  # fraction de la course du stick ignorée

    def __post_init__(self) -> None:
        if self.max_linear <= 0 or self.max_angular <= 0:
            raise ValueError("les vitesses maximales doivent être strictement positives")
        if not 0.0 <= self.deadzone < 1.0:
            raise ValueError("la zone morte doit être dans [0, 1[")


@dataclass(frozen=True)
class Velocity:
    """Consigne de vitesse d'un robot holonome (roues Mecanum)."""

    vx: float = 0.0  # avant / arrière, m/s
    vy: float = 0.0  # latéral, m/s
    wz: float = 0.0  # rotation, rad/s

    @property
    def is_zero(self) -> bool:
        return self.vx == 0.0 and self.vy == 0.0 and self.wz == 0.0


STOP = Velocity()


def clamp(value: float, low: float, high: float) -> float:
    """Borne ``value`` dans ``[low, high]``."""
    return max(low, min(high, value))


def apply_deadzone(value: float, deadzone: float) -> float:
    """Annule les petites valeurs et ré-échelonne le reste pour garder une réponse continue.

    Sans ré-échelonnage, la consigne sauterait de 0 à ``deadzone`` en sortie de zone morte.
    """
    value = clamp(value, -1.0, 1.0)
    magnitude = abs(value)
    if magnitude <= deadzone:
        return 0.0
    scaled = (magnitude - deadzone) / (1.0 - deadzone)
    return scaled if value > 0 else -scaled


def axes_to_velocity(
    forward: float,
    strafe: float,
    turn: float,
    *,
    speed_scale: float = 1.0,
    emergency_stop: bool = False,
    limits: TeleopLimits | None = None,
) -> Velocity:
    """Convertit l'état de la manette en consigne de vitesse bornée.

    ``speed_scale`` (0 à 1) module toutes les vitesses, par exemple via une gâchette.
    L'arrêt d'urgence est prioritaire sur toute autre entrée.
    """
    if emergency_stop:
        return STOP

    lim = limits or TeleopLimits()
    scale = clamp(speed_scale, 0.0, 1.0)

    vx = apply_deadzone(forward, lim.deadzone) * lim.max_linear * scale
    vy = apply_deadzone(strafe, lim.deadzone) * lim.max_linear * scale
    wz = apply_deadzone(turn, lim.deadzone) * lim.max_angular * scale

    # Éviter les -0.0, plus lisibles dans les journaux et les tests.
    return Velocity(vx=vx + 0.0, vy=vy + 0.0, wz=wz + 0.0)
