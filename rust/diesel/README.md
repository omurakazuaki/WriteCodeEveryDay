# Diesel

## Install diesel-cli
```
cargo install diesel_cli --no-default-features --features "sqlite-bundled"
```

## Setup
```
diesel setup
```

## Migrate
```
diesel migration generate create_posts
```

```
diesel migration run
```
