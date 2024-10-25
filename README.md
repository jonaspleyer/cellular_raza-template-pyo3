<div align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/jonaspleyer/cellular_raza/refs/heads/master/cellular_raza/logos/cellular_raza_dark_mode.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/jonaspleyer/cellular_raza/refs/heads/master/cellular_raza/logos/cellular_raza.svg">
        <img alt="The cellular_raza logo" src="doc/cellular_raza.svg">
    </picture>
</div>

# cellular_raza-template-pyo3
[![License: GPL 2.0](https://img.shields.io/github/license/jonaspleyer/cellular_raza-template-pyo3?style=flat-square)](https://opensource.org/license/gpl-2-0/)
[![Test](https://img.shields.io/github/actions/workflow/status/jonaspleyer/cellular_raza-template-pyo3/test_stable.yml?label=Test&style=flat-square)](https://github.com/jonaspleyer/cellular_raza/actions)


## Usage

We rely on [pyo3](https://pyo3.rs) and [maturin](https://www.maturin.rs/tutorial) to build this
package.
We require a working virtual environment where the package can be installed.
See [venv](https://docs.python.org/3/library/venv.html) for how to do this on your platform.

```bash
# Compiles and installs the package in the active virtual environment.
maturin develop -r
```

Afterwards, the package can be used in any python script

```python
>>> import cellular_raza_template_pyo3 as crt
```

For more information see the [guide on python bindings](https://cellular-raza.com/guides) for
`cellular_raza`.

## List of files to change before using this template

```bash
[3] ├── Cargo.toml
[1] ├── cellular_raza_template_pyo3 << change folder name
[1] │   └── __init__.py
[ ] ├── docs
[1] │   ├── conf.py
[2] │   ├── index.rst
[ ] │   ├── make.bat
[ ] │   ├── Makefile
[ ] │   ├── references.bib
[ ] │   ├── requirements.txt
[ ] ├── examples
[1] │   └── basic.py
[ ] ├── LICENSE
[ ] ├── make.bat
[1] ├── pyproject.toml
[1] ├── README.md
[ ] ├── requirements.txt
[ ] └── src
[ ]     ├── lib.rs
[ ]     └── main.rs
```

| File/Folder | Change |
|:---:| --- |
| `Cargo.toml` | Change `name = ...` attributes under `[package]` and `[lib]` |
|  | Change the dependency `cellular_raza = ...` to the most recent version. |
| `cellular_raza_template_pyo3` | Change folder name and import statement in `__init__.py` |
| `docs/conf.py` | Change `project = '...'` |
| `docs/index.rst` | Change first directive `.. cellular_raza_template_pyo3 ...` |
| | `cellular_raza_template_pyo3`<br>`========...` |
| `examples/basic.py` | Change import statement `import cellular_raza_template_pyo3 as crt` |
| `pyproject.toml` | Change `name = ...` |
| `README.md` | Change title and remove this table when done. |
| `src/lib.rs` | Change name of module `fn cellular_raza_template_pyo3_rs(m: ...` Notice that we use the suffix `_rs` to indicate that this is the rust-specific module. This is optional but needs to be adjusted in the corresponding `__init__.py` file. |
