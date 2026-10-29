"""Lance la téléopération : lecture manette (joy) + conversion en consigne (explorer_teleop).

Frontière matérielle (ADR 0003) : le code d'explorer-1 publie sur le topic générique
``cmd_vel``. Le remappage vers le topic réel du robot se fait ICI et nulle part ailleurs.
Sur le MentorPi M1 (pile HiWonder), ce topic est ``/controller/cmd_vel``.

Usage :
    ros2 launch explorer_bringup teleop.launch.py
    ros2 launch explorer_bringup teleop.launch.py cmd_vel_topic:=/cmd_vel   # simulation
"""

from __future__ import annotations

import os

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node


def generate_launch_description() -> LaunchDescription:
    params = os.path.join(get_package_share_directory("explorer_bringup"), "config", "teleop.yaml")
    cmd_vel_topic = LaunchConfiguration("cmd_vel_topic")

    return LaunchDescription(
        [
            DeclareLaunchArgument(
                "cmd_vel_topic",
                default_value="/controller/cmd_vel",
                description="Topic de commande de vitesse du robot cible.",
            ),
            Node(
                package="joy",
                executable="joy_node",
                name="joy_node",
                parameters=[params],
            ),
            Node(
                package="explorer_operator",
                executable="teleop_node",
                name="explorer_teleop",
                parameters=[params],
                remappings=[("cmd_vel", cmd_vel_topic)],
            ),
        ]
    )
