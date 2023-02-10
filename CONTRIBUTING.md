# Contributing

## Getting Started Locally

### Slack

Add your bot token and channel ID as environment variables to run tests.

```sh
export SLACK_BOT_TOKEN=xoxb-0000000000000000000000
export SLACK_CHANNEL_ID=C0000000000
```

See the readme for [more details](README.md#setting-up-slack)

### Compiling

```
cargo build
```

See [cargo-build](https://doc.rust-lang.org/cargo/commands/cargo-build.html).

Alternatively, as this is a library and not a binary, a plugin such as
[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) will
run cargo within your IDE and notify you of any compile errors.

### Testing

```
cargo test
```

See [cargo-test](https://doc.rust-lang.org/cargo/commands/cargo-test.html).
