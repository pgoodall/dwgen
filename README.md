![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/pgoodall/dwgen/.github%2Fworkflows%2Frust.yml?logo=github) [![CodeQL](https://github.com/pgoodall/dwgen/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/pgoodall/dwgen/actions/workflows/github-code-scanning/codeql) ![GitHub License](https://img.shields.io/github/license/pgoodall/dwgen?link=https%3A%2F%2Fgithub.com%2Fpgoodall%2Fdwgen%2Fblob%2FLICENSE)


## dwgen
This is a (very) simple utility for creating a [Diceware](https://en.wikipedia.org/wiki/Diceware)-like password by choosing a set (5 by default) of random words from a large body of text. 

This is the first release, so it is not yet very sophisticated, but I will continually improve it. I originally wrote this as a Python script about 10 years ago. The original version took me about two weeks to create, but this Rust-based version took about half a day. It is also _much_ faster. 

The latest version is `v0.1.1`.

```shell
$ dwgen --help
Usage: dwgen [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>      Name of local file
  -n, --number <NUMBER>  Number of words
  -h, --help             Print help
  -V, --version          Print version
```

