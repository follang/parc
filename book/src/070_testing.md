# Testing

## Test structure

- `src/tests/` — unit tests for parser internals
- `test/reftests/` — reference test corpus (small C fragments)
- `test/full_apps/` — larger translation-unit fixtures

## Running tests

```sh
make build   # build the library
make test    # run all tests (including ignored/system tests)
```

## Full-app fixtures

Each fixture has a `fixture.toml` manifest with fields like `mode`, `flavor`, `entry`, and `expected`.

Filter by path or tag:

```sh
FULL_APP_FILTER=musl/stdint make test
FULL_APP_TAG=synthetic make test
```

## External fixtures

Vendored headers live under `test/full_apps/external/` with provenance documented in `EXTERNAL_SOURCES.md` and licenses in `test/full_apps/licenses/`.
