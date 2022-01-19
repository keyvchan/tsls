# tsls

[Tree-sitter](https://github.com/tree-sitter/tree-sitter) based language server for general languages.

**Warning**: It's in active development right now, and bug is expected.

## Features

- [x] Go To Definition
- [x] Find References
- [x] Basic Diagnostics
- [x] AST based Completion with Scope.
- [x] Incremental Document Syncing

## Limitations

- Single file only
- Use queries from [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter/tree/master/queries)(_it's hard-coded :(_ ). so it must be preinstalled by `packer.nvim` for now.

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
