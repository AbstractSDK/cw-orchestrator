docs:
  cd docs && mdbook serve --open --port 5000

watch: 
  cargo watch -x lcheck
  
test:
  cargo test
