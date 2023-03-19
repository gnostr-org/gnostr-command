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

GIT_USER_NAME                           := $(shell git config user.name)
export GIT_USER_NAME
PACKAGE_PREFIX                          := ghcr.io
export PACKAGE_PREFIX

-:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.PHONY: help
help:## 	print verbose help
	@echo ''
	@echo 'make help'
	@echo '                  print this verbose help'
	@echo 'make report'
	@echo '                  print some make variables'
	@echo 'make docker-start'
	@echo '                  additional help'
	@echo '                  additional help'
	@echo 'make docker-pull'
	@echo '                  additional help'
	@echo '                  additional help'
	@sed -n 's/^## //p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/## /'

.PHONY: report
report:## 	print some variables
	@echo ''
	@echo 'HOME=${HOME}'
	@echo 'PWD=${PWD}'
	@echo 'PROJECT_NAME=${PROJECT_NAME}'
	@echo 'TIME=${TIME}'
	@echo 'DOCKER=${DOCKER}'
	@echo 'DOCKER_COMPOSE=${DOCKER_COMPOSE}'
	@echo ''

.ONESHELL:
docker-start:## 	detect whether docker is running...
	@( \
	    while ! docker system info > /dev/null 2>&1; do\
	    echo 'Waiting for docker to start...';\
	    if [[ '$(OS)' == 'Linux' ]]; then\
	     systemctl restart docker.service;\
	    fi;\
	    if [[ '$(OS)' == 'Darwin' ]]; then\
	     open --background -a /./Applications/Docker.app/Contents/MacOS/Docker;\
	    fi;\
	sleep 1;\
	done\
	)
docker-pull:docker-start## 	pull alpine image
	docker pull alpine
run:needle-haystack
needle-haystack:
	@gnostr-git config advice.addIgnoredFile false
	@gnostr-git add .gnostr/* ./src *akefile *.mk
	cargo test
	RUST_BACKTRACE=1 cargo run -- the poem.txt
	RUST_BACKTRACE=1 cargo run -- frog poem.txt
	RUST_BACKTRACE=1 cargo run -- body poem.txt
	RUST_BACKTRACE=1 cargo run -- monomorphization poem.txt
	#RUST_BACKTRACE=1 cargo run --     poem.txt
	@$(MAKE) gnostr-legit
gnostr-legit:
	gnostr-git status
	@gnostr-legit
	@gnostr-git add .gnostr
	@git config -l | grep "gnostr" > .gnostr/gnostr.relays

-include cargo.mk
