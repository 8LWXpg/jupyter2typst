# Jupyter to Typst converter

Feel free to open a issue or PR!

## Features

- automatic image download/extract
- nice template
- support for KaTeX in markdown convert (not complete)

Look for demos in [test](test) folder.

## Install

Install `jupyter2typst` with cargo:

```bash
cargo install --git 'https://github.com/8LWXpg/jupyter2typst.git'
```

After that, you can use `jupyter2typst` in your terminal.

Don't forget to download the [template](https://github.com/8LWXpg/jupyter2typst/blob/master/test/template.typ) and include it in the same directory as your `.ipynb` file.

```bash
curl 'https://raw.githubusercontent.com/8LWXpg/jupyter2typst/master/test/template.typ' > template.typ
```

modify the `template.typ` to fit your needs.

## Usage

```bash
jupyter2typst <input> [-o <output>] [-i <img-path>]
```

- `input`: the path of the input `.ipynb` file
- `output`: the path of the output `.typ` file
- `img-path`: the path of the image folder, default is `./img`

## KaTeX support list

read [convert_list.md](convert_list.md)
