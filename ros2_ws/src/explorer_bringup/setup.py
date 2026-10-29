from glob import glob

from setuptools import setup

PACKAGE = "explorer_bringup"

setup(
    name=PACKAGE,
    version="0.1.0",
    packages=[],
    data_files=[
        ("share/ament_index/resource_index/packages", [f"resource/{PACKAGE}"]),
        (f"share/{PACKAGE}", ["package.xml"]),
        (f"share/{PACKAGE}/launch", glob("launch/*.launch.py")),
        (f"share/{PACKAGE}/config", glob("config/*.yaml")),
    ],
    install_requires=["setuptools"],
    zip_safe=True,
    maintainer="Thomas Blondel",
    description="Fichiers de lancement et paramètres d'explorer-1.",
    license="MIT",
)
