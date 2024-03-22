============
Run commands
============

When you want release new version, update version contents by age subcommands.

``init``
========

Initialize age environnment to create config file.

Options
-------

``--preset``
  Pattern of replacement targes (can accepts multiple values).
  Currently, support values are ``rust`` and ``python``.

Example
-------

.. code:: console

   age init --preset rust

``update``
==========

Update version and edit rarget files.

``major``
=========

Shortcut subcommand of ``update`` for major version updating.

If ``current_version`` is "1.2.3", this command changes version for "2.0.0".

``minor``
=========

Shortcut subcommand of ``update`` for minor version updating.

If ``current_version`` is "1.2.3", this command changes version for "1.3.0".

``patch``
=========

Shortcut subcommand of ``update`` for patch version updating.

If ``current_version`` is "1.2.3", this command changes version for "1.2.4".

``info``
========

Display information of configuation file.
