## Audio Plugins

### 1. Take My Headphones (Crossfeed)

Functional emulation of the Matrix section of the SPL Phonitor 3 and more.

![screenshot](./docs/take_my_headphones.png)

### Contributing

```bash
git submodule update --init --recursive

cd external/clap-1.2.7
git checkout tags/1.2.7

cd external/neural-amp-modeler-0.5.1
git checkout tags/v0.5.1
```

### Releases

Uses [`cargo-release`](https://github.com/crate-io/cargo-release). Config in `release.toml`.

**Release a plugin:**

```bash
cargo release --package <crate-name> [patch|minor|major]
# Example:
cargo release --package take-my-headphones patch
```

What it does automatically:

1. Bumps version in `Cargo.toml` according to the level
2. Commits with `chore: bump <crate-name> to <version>`
3. Creates git tag `<crate-name>-v<version>`
4. Pushes commit + tag to remote

No crates.io publish (plugins, not libraries). CI runs tests — `verify = false` in config skips local test run before release.
