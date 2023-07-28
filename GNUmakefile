SHELL := /bin/bash

PWD                                     ?= pwd_unknown
PROJECT_NAME                            := $(notdir $(PWD))
THIS_FILE                               := $(lastword $(MAKEFILE_LIST))
export THIS_FILE
TIME                                    := $(shell date +%s)
export TIME

OS                                      :=$(shell uname -s)
export OS

ifeq ($(docker),)
DOCKER                                  := $(shell which docker)
else
DOCKER                                  := $(docker)
endif
export DOCKER
ifeq ($(compose),)
DOCKER_COMPOSE                          := $(shell which docker-compose)
else
DOCKER_COMPOSE                          := $(compose)
endif
export DOCKER_COMPOSE
RUSTC                                   :=$(shell which rustc)
export RUSTC
RUSTUP                                  :=$(shell which rustup)
export RUSTUP
RUSTUP_INIT                             :=$(shell which rustup-init)
export RUST_INIT
CARGO                                   :=$(shell which cargo)
export CARGO

GIT_USER_NAME                           := $(shell git config user.name)
export GIT_USER_NAME
PACKAGE_PREFIX                          := ghcr.io
export PACKAGE_PREFIX

-:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.PHONY: help
help:## 	more help
##help
	@echo ''
	@echo 'additional help'
	@sed -n 's/^##/ /p' ${MAKEFILE_LIST} |  sed -e 's/^//'

.PHONY: report
report:## 	print some variables
##
	@echo ''
	@echo 'HOME=${HOME}'
	@echo 'PWD=${PWD}'
	@echo 'PROJECT_NAME=${PROJECT_NAME}'
	@echo 'TIME=${TIME}'
	@echo 'DOCKER=${DOCKER}'
	@echo 'DOCKER_COMPOSE=${DOCKER_COMPOSE}'
	@echo 'RUSTC=${RUSTC}'
	@echo 'RUSTUP=${RUSTUP}'
	@echo 'RUSTUP_INIT=${RUSTUP_INIT}'
	@echo 'CARGO=${CARGO}'
	@echo ''

install:cargo-install## 	
##install: cargo-install
run-tests:## 	run-tests
##run tests
##	make gnostr-command
	@$(MAKE) gnostr-command
##	cargo test
	cargo test
##	RUST_BACKTRACE=1 cargo run -- gnostr README.md
	RUST_BACKTRACE=1 cargo run -- gnostr README.md
##	RUST_BACKTRACE=1 cargo run -- relays .git
	RUST_BACKTRACE=1 cargo run -- relays .git
gnostr-command:cargo-install##
##gnostr-command
	@which gnostr-command


-include docker.mk
-include cargo.mk
