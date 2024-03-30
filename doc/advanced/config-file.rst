=========================
Extra configuration files
=========================

age supports other files excepted for `.age.toml` to manage configuration.

Order of finding
================

age check "these are exiets" and "this includes configuration of age".
It use first valid file for configuration .

1. ``.age.toml``
2. ``Cargo.toml``
3. ``pyproject.toml``

File and section
================

``.age.toml``
-------------

Standard configuration file of ``age``.

Configuration values are managed on top-level sesciont.

``Cargo.toml``
--------------

Manifest file of Rust project.

``age`` refers ``package.metadata.age`` of file.

.. note:: See spec of file on `"The Cargo Book" <https://doc.rust-lang.org/cargo/reference/manifest.html>`_.

``pyproject.toml``
------------------

Metadata management file of Python project.

``age`` refers ``tool.age`` of file.

.. note:: See `spec of file <https://packaging.python.org/en/latest/specifications/pyproject-toml/>`_.
