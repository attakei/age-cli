===========
Install age
===========

At first, you need install age into your machine.

Patterns
========

You can age by some approach.

Get built binary
----------------

You can download and use from GitHub Releases.

.. code:: console

   curl -L https://github.com/attakei/age-cli/releases/download/v0.4.0/age-v0.4.0_linux.zip | bsdtar x -
   chmod +x age-v0.4.0/age
   cp age-v0.4.0/age /path/to/bin/

Using Cargo
-----------

You can install by ``cargo install`` command,
if you are user or developer of Rust-lang and there is rust environment on your mathine.

This is suitable for:

* Want to manage cargo-driven files on single workspace.
* Want to use with latest source.

.. code:: console

   cargo install --git https://github.com/attakei/age-cli

After install
=============

Please try running ``age`` command to check installed rightly.

.. code:: console

   age version

When console displays version-text, Installation is successfully!!
