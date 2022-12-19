default: install

all: hooks install build


h help:
	@grep '^[a-z]' Makefile


install:
	cargo install mdbook


s serve:
	cd docs; \
	mdbook serve


build:
	cd docs; \
	mdbook build