.PHONY: build

NAME     := turbofish
IMG_NAME := btwiuse/${NAME}
IMAGE    := ${IMG_NAME}
PACKAGE  := github.com/${IMG_NAME}
GIT      := $(shell git rev-parse --short HEAD)
DATE     := $(shell date +%FT%T%Z)
SHELL    := bash
BAZEL    := ./bazelw

default: help

raze:             ## auto generate BUILD.bazel files from Cargo.toml
	@ which cargo-raze || (echo please cargo install cargo-raze | grep --color . 1>&2 && false)
	@ cargo vendor --versioned-dirs cargo/vendor &>/dev/null
	@ cargo raze
	@ git status cargo

bazel-build:          ## Build binary for current platform using bazel
	@ $(BAZEL) build //:turbofish # //cmd/{hub,client,agent}

build:          ## Build binary for current platform
	@ cargo build --release

clean:          ## Clean build artifacts
	rm -r bin

test:           ## Run all tests
	@go clean --testcache && go test ./...


img:            ## Build Docker Image
	@docker build --rm -t ${IMAGE} .

bazel-img:      ## Build Docker Image with bazel
	@ ${BAZEL} build //:k0s_image
	@ docker load -i bazel-bin/k0s_image.tar
	@ docker tag bazel:k0s_image ${IMAGE}

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":[^:]*?## "}; {printf "\033[38;5;69m%-30s\033[38;5;38m %s\033[0m\n", $$1, $$2}'
