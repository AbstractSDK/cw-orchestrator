test:
  cargo test

install-docs:
  cargo install mdbook

serve-docs:
  (cd docs && mdbook serve --open) 

build-docs:
  (cd docs && mdbook build)

doc-test:
  cargo test --doc
  mdbook test docs
