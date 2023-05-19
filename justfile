test:
  cargo test

install-docs:
  cargo install mdbook

serve-docs:
  (cd docs && mdbook serve --open) 

build-docs:
  (cd docs && mdbook build)

fmt:
  (cargo fmt && find . -type f -iname "*.toml" -print0 | xargs -0 taplo format)
