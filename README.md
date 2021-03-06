# tsls

[![license](http://img.shields.io/badge/license-Apache%20v2-blue.svg)](https://raw.githubusercontent.com/keyvchan/tsls/main/LICENSE)
[![build](https://github.com/Peltoche/lsd/workflows/CICD/badge.svg)](https://github.com/keyvchan/tsls/actions)

[Tree-sitter](https://github.com/tree-sitter/tree-sitter) based language server for general languages.

**Warning**: It's in active development right now, and bug is expected.

## Features

- [x] Go To Definition
- [x] Find References
- [x] Basic Diagnostics
- [x] AST based Completion with Scope
- [x] Incremental Document Syncing
- [x] Smart Rename In single file

## Future Plans

- Generalize project layout abstraction, enable project wised analysis.
- Add support for more languages.
- Out-of-box experience.

## Support Languages

For supported languages, check out [Support Matrix](https://github.com/keyvchan/tsls/wiki/Support-Matrix)

## Limitations

- Single file only

## Build

```bash
 $ git clone https://github.com/keyvchan/tsls
 $ cd tsls
 $ git submodule update --init --recursive
 $ cargo build
```

## Inspired by

- [semantic](https://github.com/github/semantic)
- [bash-language-server](https://github.com/bash-lsp/bash-language-server)
- [rust-analyzer](https://github/github.com/rust-analyzer/rust-analyzer)
- [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter)
- [nvim-treesitter-refactor](https://github.com/nvim-treesitter/nvim-treesitter-refactor)
