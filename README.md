# README

## Dev build

1. Start server on port :8000

```
$ cargo run --bin recipe-server
```

1. Serve UI using Dioxus CLI with hot-reload enabled

```
$ (cd crates/recipe-web && dioxus serve --hot-reload)
```

## Prod build

1. Build web assets

```
$ (cd crates/recipe-web && trunk build)
```

1. Run server inside Shuttle

```
$ (cd crates/recipe-shuttle && cargo shuttle run)
```
