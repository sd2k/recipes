#!/usr/bin/env just --justfile

export RUST_LOG := "tower-http=debug,dioxus=debug,recipe=debug,info"

default:
  @just --list

run-desktop:
  cd crates/recipe-desktop && cargo run

build-web:
  cd crates/recipe-web && dx build
  
run-server: build-web
  cd crates/recipe-server && cargo run

