from setuptools import find_packages, setup

PACKAGE = "explorer_operator"

setup(
    name=PACKAGE,
    version="0.1.0",
    packages=find_packages(exclude=["test"]),
    data_files=[
        ("share/ament_index/resource_index/packages", [f"resource/{PACKAGE}"]),
        (f"share/{PACKAGE}", ["package.xml"]),
    ],
    install_requires=["setuptools", "pyyaml"],
    zip_safe=True,
    maintainer="Thomas Blondel",
    description="Côté opérateur d'explorer-1 : configuration, téléopération, interface.",
    license="MIT",
    tests_require=["pytest"],
    entry_points={
        "console_scripts": [
            "teleop_node = explorer_operator.teleop_node:main",
            "operator_app = explorer_operator.app:main",
        ],
    },
)
