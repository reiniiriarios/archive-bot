# Archive Bot

Slack bot helper for managing outdated and very small channels.

## Using

This bot is not currently on the Rust registry. To add to a project, add the following to your `[dependencies]` in `Cargo.toml`.

```toml
archive_bot = { git = "https://github.com/reiniiriarios/archive-bot" }
```

See the [examples](examples/) directory for implementation details.

## Configuration

Archive bot needs a bit of data to get started:

- Slack API Key
- Notification [Channel ID](#finding-slack-channel-id)
- Filter Prefixes (optional)
- Messages (optional)
- Staleness (optional)
- Small Channel Threshold (optional)

```rust
let config = archive_bot::Config {
  token: env::var("SLACK_BOT_TOKEN").expect("Error: environment variable SLACK_BOT_TOKEN is not set."),
  notification_channel_id: "A01A02A03A04",
  filter_prefixes: vec!["-"],
  message_headers: vec![
    "Hey, you've got some cleaning up to do!",
    "Hey boss, take a look at these, will ya?",
  ],
  stale_after: 2 * 7 * 24 * 60 * 60,
  small_channel_threshold: 3,
};
```

Or, using default values:

```rust
let config = archive_bot::Config {
  token: env::var("SLACK_BOT_TOKEN").expect("Error: environment variable SLACK_BOT_TOKEN is not set."),
  notification_channel_id: "A01A02A03A04",
  ..archive_bot::Config::default()
};
```

## Setting Up Slack

-- TODO --

Generate your Bot User OAuth Token on the [Slack API Admin](https://api.slack.com/apps) > \[Your App\] > Features > OAuth & Permissions.

Your Slack Bot needs the following scopes:

- `channels:history`
- `channels:join`
- `channels:read`
- `chat:write`
- `groups:history`
- `groups:read`

## Logging

Archive Bot implements the [log](https://docs.rs/log/latest/log/) crate and does not produce output directly.
See the [examples](#) directory for an implementation of [simplelog](https://github.com/drakulix/simplelog.rs).

## Finding Slack Channel ID

To find the ID of a Slack channel, you can click the channel name for more info and find it at the bottom.

<img src="docs/find-channel-id.png" alt="Screenshot of Slack channel info with an arrow pointing to the Channel ID at the bottom of the window." width="500">
