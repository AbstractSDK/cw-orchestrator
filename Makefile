default: install

all: hooks install build


h help:
	@grep '^[a-z]' Makefile


install:
	cargo install mdbook


s serve:
	mdbook serve


build:
	mdbook build