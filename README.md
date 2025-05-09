# Jupyter to Typst converter

Jupyter to Typst converter with template support.

## Features

- Automatic image download/extract
- Nice template
- Support for KaTeX in markdown math (mostly complete)

Look for demos in [test](test) folder.

## Install

### Download

Download binary from [release](https://github.com/8LWXpg/jupyter2typst/releases/latest) page

### Install with `cargo-binstall`

```bash
cargo binstall --git https://github.com/8LWXpg/jupyter2typst jupyter2typst
```

### Install with `cargo`

```bash
cargo install --git 'https://github.com/8LWXpg/jupyter2typst.git' --features native-tls
```

After that, you can use `jupyter2typst` in your terminal.

## Template

The output `.typ` file imports a template file. It is required to have a `template.typ` file in the same directory as the output `.typ` file.

Check the [template list](./template/template.md) for templates others have made.

Download the [template](./template/template.typ) file:

```bash
wget https://raw.githubusercontent.com/8LWXpg/jupyter2typst/master/template/template.typ
```

Modify the `template.typ` to fit your needs.

## Usage

```bash
jupyter2typst <input> [-o <output>] [-i <img-path>]
```

- `input`: the path of the input `.ipynb` file
- `output`: the path of the output `.typ` file
- `img-path`: the path of the image folder, default is `./img`

## Contributing

### Adding a template

1. Create a new template file `<template-name>.typ` in the `template` folder
2. Add simple description in `template/template.md`
3. Submit a PR with a preview image of `template/base.typ` compiled with the new template

## KaTeX support list

read [convert_list.md](convert_list.md)
