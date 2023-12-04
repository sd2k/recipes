#!/usr/bin/env just --justfile

export RUST_LOG := "tower_http=debug,dioxus=debug,recipe=debug,info"

alias d := run-desktop
alias s := run-server
alias w := build-web

default:
  @just --list

run-desktop:
  cd crates/recipe-desktop && cargo run

build-web:
  cd crates/recipe-web && dx build
  
run-server: build-web
  cd crates/recipe-server && cargo run

run-shuttle: build-web
  cd crates/recipe-shuttle && cargo shuttle run

doc:
  cargo doc --open
