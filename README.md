# Archive Bot

Slack bot helper for managing outdated and very small channels. In its current iteration, it runs
once and ends, meaning it is meant to be run on a schedule, such as a cron job or a Lambda function.

## Configuration

Archive bot needs a bit of data to get started:

- Slack [Bot Token](https://api.slack.com/authentication/token-types#bot)
- Notification [Channel ID](https://github.com/reiniiriarios/archive-bot/#finding-slack-channel-id)
- Filter Prefixes (optional)
    - The bot will ignore channels with these prefixes.
- Messages (optional)
    - Configure messages to send prefixing updates.
- Staleness (optional)
    - Configure how long a channel has to go without a message before it's considered "old."
- Small Channel Threshold (optional)
    - Configure how small a channel has to be before it's considered "small."

```rust
let bot = ArchiveBot {
  // Bot tokens look like: xoxb-xxxxxxxyourtokenxxxxxxx.
  token: env::var("SLACK_BOT_TOKEN").expect("Error: environment variable SLACK_BOT_TOKEN is not set."),
  // Use the channel ID and not the name.
  notification_channel_id: env::var("SLACK_CHANNEL_ID").expect("Error: environment variable SLACK_CHANNEL_ID is not set."),
  // Ignore channels beginning with these prefixes.
  filter_prefixes: vec!["-"],
  // Messages to send (one is picked at random).
  message_headers: vec![
    "Hey, you've got some cleaning up to do!",
    "Hey boss, take a look at these, will ya?",
  ],
  // How long before a channel is stale (in seconds).
  stale_after: 6 * 7 * 24 * 60 * 60,
  // How small a "small" channel is.
  small_channel_threshold: 3,
  // Whether to send a secondary notification to a different channel (message only).
  notify_secondary_channel: true,
  // The ID of a secondary channel.
  secondary_notification_channel_id:  env::var("SLACK_CHANNEL_2_ID").expect("Error: environment variable SLACK_CHANNEL_2_ID is not set."),
  // The message prefix to send to the secondary channel. Will be suffixed with a link to the primary channel.
  secondary_message_headers: vec![
    "Hey folks! I, uh... made a list for you. Of channels. That you should archive. Maybe.",
    "Hey everyone! If you want the satisfaction of crossing a task off your list, I have one!",
  ],
  ..ArchiveBot::default()
};
```

Or, using default values:

```rust
let bot = ArchiveBot {
  token: env::var("SLACK_BOT_TOKEN").expect("Error: environment variable SLACK_BOT_TOKEN is not set."),
  notification_channel_id: "C01A02A03A04".to_string(),
  ..ArchiveBot::default()
};
```

## Implementation

Currently this bot consists of a single runtime, with a single action. Further actions and
config options TBD.

```rust
match bot.run().await {
  Ok(_) => println!("Success!"),
  Err(e) => panic!("Uhoh! {:}", e),
}
```

See the [examples](https://github.com/reiniiriarios/archive-bot/examples/) directory for further implementation details.

## Setting Up Slack

See Slack documentation for [basic app setup](https://api.slack.com/authentication/basics).

Generate your Bot User OAuth Token on the [Slack API Admin](https://api.slack.com/apps) > \[Your App\] > Features > OAuth & Permissions.

Your app needs the following scopes:

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
