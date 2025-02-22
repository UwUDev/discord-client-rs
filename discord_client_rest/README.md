# Discord Client Rest

A high-level Rust implementation of the Discord REST API, designed to provide a robust and efficient client-side interface to Discord's HTTP API.

This crate offers a seamless integration for Discord bot developers, featuring:

- **Automatic ratelimit handling** for all REST requests
- **TLS impersonation** and **HTTP/2 (H2) mimicing** of Chrome to avoid detection
- Automated token login and session management
- Efficient handling of Discord's REST responses
- Captcha detection and handling
- Automatic headers generation for Discord's REST requests

## Usage

### Dependencies

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
discord_client_rest = "0.1.0"
```

### Creating a client

```rust
let token = ...

let custom_api_version = None;
let custom_build_number = None;

let mut client = RestClient::connect(token, custom_api_version, custom_build_number)
    .await
    .unwrap();
```

### getting API version and build number

```rust
println!("API Version: {}", client.api_version);

// Useful for the gateway client
println!("Build Number: {}", client.build_number);
```

### Examples

- **[Channel](examples/channel.rs)**
- **[DM](examples/dm.rs)**
- **[Group](examples/group.rs)**
- **[Message](examples/message.rs)**


### Making custom requests

```rust
let path = "/channels/1234567890/messages";

// alwasy make sur to add a referer
let referer: Referer = match guild_id {
    Some(guild_id) => GuildChannelReferer {
        guild_id,
        channel_id: self.channel_id,
    }
    .into(),
    None => DmChannelReferer {
        channel_id: self.channel_id,
    }
    .into(),
};

let props = RequestPropertiesBuilder::default()
    .referer::<Referer>(referer.into())
    // .context(....)
    // .solved_captcha(...)
    .build()?;

// Any struct that implements Serialize can be used as the body
let body = MessageBuilder::default()
    .content("Hello, world!")
    .build()?;

// This will automatically handle ratelimits and return the response
// The return type is an Option<T> where T is Deserialize
let resp: ? = self.client
            .post::<Message, Message>(&path, Some(message), Some(props))
            .await?;
```

You call also use `put`, `patch`, `delete`, `get` methods to make requests.

The `get` method is used to make requests that cannot send a body but instead can receive query parameters.

```rust
let path = format!("channels/{}/messages/search", self.channel_id);

let referer = DmChannelReferer {
    channel_id: self.channel_id,
};

let props = RequestPropertiesBuilder::default()
    .referer::<Referer>(referer.into())
    .build()?;

// Format is HashMap<String, String> for the query
self.client
    .get::<MessageSearchResult>(&path, Some(query.to_map()), Some(props))
    .await
```
