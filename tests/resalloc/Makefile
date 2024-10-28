.PHONY: all help clean cleanall view format

SHELL=/usr/bin/env bash -eo pipefail

.SECONDARY:

.SUFFIXES:

##############
## Commands ##
##############

all:
	cargo build --release

format:
	cargo fmt --all


help: ## Print help messages
	@echo -e "$$(grep -hE '^\S*(:.*)?##' $(MAKEFILE_LIST) \
		| sed \
			-e 's/:.*##\s*/:/' \
			-e 's/^\(.*\):\(.*\)/   \\x1b[36m\1\\x1b[m:\2/' \
			-e 's/^\([^#]\)/\1/g' \
			-e 's/: /:/g' \
			-e 's/^#\(.*\)#/\\x1b[90m\1\\x1b[m/' \
		| column -c2 -t -s : )"

clean: ## Clean

cleanall: clean ## Clean all

view: ## View output
