#!/usr/bin/env just --justfile

export RUST_LOG := "tower_http=debug,dioxus=debug,recipe=debug,info"

alias d := run-desktop
alias s := run-server
alias w := build-web
alias ww := watch-web
alias tw := tailwind
alias wtw := watch-tailwind

default:
  @just --list

build-desktop:
  cd crates/recipe-desktop && dx build --platform desktop

run-desktop: build-desktop
  cd crates/recipe-desktop && cargo run

watch-desktop: build-desktop
  cd crates/recipe-desktop && cargo watch -x run

build-web:
  cd crates/recipe-web && dx build

watch-web:
  cd crates/recipe-web && cargo watch -s 'dx build'
  
run-server: build-web
  cd crates/recipe-server && cargo run

watch-server: build-web
  cd crates/recipe-server && cargo watch -x run

run-shuttle: build-web
  cd crates/recipe-shuttle && cargo shuttle run

watch-shuttle: build-web
  cd crates/recipe-shuttle && cargo watch -x 'shuttle run'

tailwind:
  cd crates/recipe-app && npx tailwindcss -i ./input.css -o ./public/tailwind.css

watch-tailwind:
  cd crates/recipe-app && npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch

doc:
  cargo doc --open
