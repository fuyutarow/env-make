# env-make

WIP

- [x] path
- [x] alias
- [x] env

## Installation

```
brew intall fuyutarow/tap/env-make
```

## Usage

### `env-make build`
```
env-make build --to nu > ~/.config/nu/config.toml
```

### `env-make install`
```
env-make install exa
```
```
env-make install --all
```

### `env-make config path`
```
code "$(env-make config path)"
```