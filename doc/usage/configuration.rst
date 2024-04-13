=============
Configuration
=============

You need setup age's configuration on your project.

First setup
===========

After install, you can get configuration file as ``.age.toml`` by running command ``age init``.

.. code-block:: console
   :caption: Simple Rust project

   age init

.. code-block:: console
   :caption: For Python project

   age init --preset python

If you want to using sample, Please see `configuration file of project itself <https://github.com/attakei/age-cli/blob/main/.age.toml>`_.

Configuration values
====================

``current_version``
-------------------

Version text managed by age.

.. warning::

   This is auto-updated value by age.
   Do not edit manually.

``files``
---------

List of target to replace by age.

.. code-block:: toml
   :caption: Simple example

   [[files]]
   path = "Cargo.toml"
   search = "version = \"{{current_version}}\""
   replace = "version = \"{{new_version}}\""

``files[].path``
----------------

File path of replacement target.
This should be relative path of configuration file.

``files[].search``
------------------

Search target of file.
This accepts multi-line text and using templating.

``files[].regex``
------------------

:Required: No
:Default: ``false``

Flag to use regular expression (regex) when searching target.

If it is ``true``, age search target using regex and replace text with captured text.

.. note:: See it: https://docs.rs/regex/1.10.4/regex/

``files[].replace``
-------------------

Replacement text for search target of file.
This accepts multi-line text and using templating.
