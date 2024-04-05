# noqa: D100
# Configuration file for the Sphinx documentation builder.

# -- Project information
project = "age"
copyright = "2024, attakei.net"
author = "Kazuya Takei"
release = "0.6.0"

# -- General configuration
extensions = ["myst_parser", "sphinx.ext.todo"]
templates_path = ["_templates"]
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]

# -- Options for HTML output
html_theme = "piccolo_theme"
html_static_path = ["_static"]
html_css_files = ["css/custom.css"]
html_title = "age -faster bumpversion alternation-"
html_short_title = "age"
html_theme_options = {
    "globaltoc_collapse": False,
    "source_url": "https://github.com/attakei/age-cli/",
    "source_icon": "github",
}

# -- Options for extensions
# sphinx.ext.todo
todo_include_todos = True
