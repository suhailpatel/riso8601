import sys

from setuptools import setup
from setuptools_rust import RustExtension

setup_requires = [
    "setuptools-rust>=0.10.6",
    "wheel"
]

install_requires = []

setup(
    name="riso8601",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
    ],
    packages=["riso8601"],
    rust_extensions=[RustExtension("riso8601.riso8601")],
    install_requires=install_requires,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
)
