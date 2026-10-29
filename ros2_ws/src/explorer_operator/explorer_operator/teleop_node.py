"""Nœud ROS2 de téléopération : ``joy`` (sensor_msgs/Joy) → ``cmd_vel`` (geometry_msgs/Twist).

Toute la logique de calcul est dans :mod:`explorer_operator.teleop` ; ce nœud ne fait que :
1. lire les axes et boutons configurés en paramètres ROS ;
2. appeler :func:`axes_to_velocity` ;
3. publier la consigne sur ``cmd_vel`` (topic générique, remappé dans le fichier de
   lancement vers le topic réel du robot : c'est la frontière matérielle, voir ADR 0003).

La lecture de la manette elle-même est confiée au paquet standard ``joy`` (SDL2).

[TODO jalon M2] Vérifier les index d'axes et de boutons du Steam Deck et les reporter
dans ``explorer_bringup/config/teleop.yaml``.
"""

from __future__ import annotations

from collections.abc import Sequence

import rclpy
from geometry_msgs.msg import Twist
from rclpy.node import Node
from sensor_msgs.msg import Joy

from explorer_operator.teleop import TeleopLimits, Velocity, axes_to_velocity


def _axis(axes: Sequence[float], index: int, invert: bool) -> float:
    """Lit un axe de façon sûre : un index absent ou négatif vaut 0."""
    if index < 0 or index >= len(axes):
        return 0.0
    value = float(axes[index])
    return -value if invert else value


def _button(buttons: Sequence[int], index: int) -> bool:
    return 0 <= index < len(buttons) and bool(buttons[index])


class TeleopNode(Node):  # type: ignore[misc]  # rclpy n'est pas typé
    """Convertit les messages de la manette en consignes de vitesse."""

    def __init__(self) -> None:
        super().__init__("explorer_teleop")

        # Index des axes et boutons (valeurs par défaut à vérifier sur le Steam Deck).
        self._axis_forward = self.declare_parameter("axis_forward", 1).value
        self._axis_strafe = self.declare_parameter("axis_strafe", 0).value
        self._axis_turn = self.declare_parameter("axis_turn", 3).value
        self._invert_forward = self.declare_parameter("invert_forward", False).value
        self._invert_strafe = self.declare_parameter("invert_strafe", False).value
        self._invert_turn = self.declare_parameter("invert_turn", False).value
        self._button_estop = self.declare_parameter("button_estop", 3).value

        self._limits = TeleopLimits(
            max_linear=self.declare_parameter("max_linear", 0.3).value,
            max_angular=self.declare_parameter("max_angular", 1.0).value,
            deadzone=self.declare_parameter("deadzone", 0.08).value,
        )

        self._publisher = self.create_publisher(Twist, "cmd_vel", 10)
        self.create_subscription(Joy, "joy", self._on_joy, 10)
        self.get_logger().info(f"Téléopération prête (limites : {self._limits})")

    def _on_joy(self, msg: Joy) -> None:
        velocity = axes_to_velocity(
            forward=_axis(msg.axes, self._axis_forward, self._invert_forward),
            strafe=_axis(msg.axes, self._axis_strafe, self._invert_strafe),
            turn=_axis(msg.axes, self._axis_turn, self._invert_turn),
            emergency_stop=_button(msg.buttons, self._button_estop),
            limits=self._limits,
        )
        self._publisher.publish(_to_twist(velocity))

    def publish_stop(self) -> None:
        """Publie une consigne nulle."""
        self._publisher.publish(Twist())


def _to_twist(velocity: Velocity) -> Twist:
    twist = Twist()
    twist.linear.x = velocity.vx
    twist.linear.y = velocity.vy
    twist.angular.z = velocity.wz
    return twist


def main(args: list[str] | None = None) -> None:
    rclpy.init(args=args)
    node = TeleopNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        # Dernière consigne nulle pour ne pas laisser le robot sur sa lancée.
        node.publish_stop()
        node.destroy_node()
        rclpy.try_shutdown()
