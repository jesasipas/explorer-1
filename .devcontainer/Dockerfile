# Environnement de développement explorer-1 : Ubuntu 22.04 + ROS 2 Humble + outils qualité.
# Construit automatiquement par VS Code (extension Dev Containers) à partir de devcontainer.json.

FROM ros:humble-ros-base

ARG USERNAME=ros
ARG USER_UID=1000
ARG USER_GID=1000
ENV DEBIAN_FRONTEND=noninteractive

# Utilisateur non root, avec sudo sans mot de passe (pratique pour apt / rosdep dans le conteneur).
RUN groupadd --gid "${USER_GID}" "${USERNAME}" \
    && useradd --uid "${USER_UID}" --gid "${USER_GID}" --create-home --shell /bin/bash "${USERNAME}" \
    && apt-get update \
    && apt-get install -y --no-install-recommends sudo \
    && echo "${USERNAME} ALL=(root) NOPASSWD:ALL" > "/etc/sudoers.d/${USERNAME}" \
    && chmod 0440 "/etc/sudoers.d/${USERNAME}" \
    && rm -rf /var/lib/apt/lists/*

# Outils de développement et paquets ROS 2 utilisés dès les premiers jalons.
# slam_toolbox et Nav2 (lourds) seront ajoutés aux jalons M3 / M4.
RUN apt-get update && apt-get install -y --no-install-recommends \
        git \
        bash-completion \
        python3-pip \
        python3-colcon-common-extensions \
        python3-rosdep \
        ros-humble-joy \
        ros-humble-rmw-cyclonedds-cpp \
        ros-humble-rmw-zenoh-cpp \
    && rm -rf /var/lib/apt/lists/*

# Outils qualité Python (mêmes versions que la CI).
COPY requirements-dev.txt /tmp/requirements-dev.txt
RUN pip3 install --no-cache-dir -r /tmp/requirements-dev.txt && rm /tmp/requirements-dev.txt

# ROS 2 chargé automatiquement dans chaque terminal, ainsi que le workspace s'il est construit.
RUN { \
        echo 'source /opt/ros/humble/setup.bash'; \
        echo 'source /usr/share/colcon_argcomplete/hook/colcon-argcomplete.bash 2>/dev/null || true'; \
        echo 'WS="${EXPLORER_WS:-}/ros2_ws/install/setup.bash"'; \
        echo '[ -f "$WS" ] && source "$WS"'; \
    } >> "/home/${USERNAME}/.bashrc"

USER ${USERNAME}
