# README

1. Build web assets

```
$ (cd crates/recipe-web && trunk build)
```

1. Copy dist to Shuttle project directory

```
$ cp -r crates/recipe-web/dist crates/recipe-shuttle
```

1. Run server inside Shuttle

```
$ (cd crates/recipe-shuttle && cargo shuttle run)
```
