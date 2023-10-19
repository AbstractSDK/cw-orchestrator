test:
  cargo test

install-docs:
  cargo install mdbook
  cargo install mdbook-keeper@0.3.0

install-docs-ci:
  mkdir bin
  curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.4.35/mdbook-v0.4.35-x86_64-unknown-linux-gnu.tar.gz | tar -xz --directory=bin
  bin/mdbook build

setup-docs:
  cargo install mdbook-keeper

serve-docs:
  (cd docs && mdbook serve --open)

build-docs:
  (cd docs && mdbook build)

doc-test:
  cargo test --doc
  mdbook build docs

format:
  cargo fmt --all
  find . -type f -iname "*.toml" -print0 | xargs -0 taplo format


