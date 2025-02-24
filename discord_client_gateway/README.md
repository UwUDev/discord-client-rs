# Discord Client Gateway

A high-level Rust implementation of the Discord gateway, designed to provide a robust and efficient client-side connection to Discord's real-time WebSocket API.

This crate offers a seamless integration for Discord bot developers, featuring:

- **Zlib-stream decompression** support for Discord's compressed payloads
- **TLS impersonation** and **HTTP/2 (H2) mimicing** of Chrome to avoid detection
- Automatic heartbeat with sequence number handling
- Efficient handling of Discord's WebSocket events
- Automatic shard management for scalable bot implementations

Whether you're building a simple bot or a complete Discord client reimplementation, this crate provides the tools you need to establish and maintain a reliable connection to Discord's gateway.

## Key Features

- **High-level API**
- **Automatic reconnection**
- **Event dispatching**
- **Guilds subscribing**

## Usage

### Dependencies

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
discord_client_gateway = "0.1.0"
```

### Creating a client

```rust
let token = "CLIENT_TOKEN".to_string();
let capabilities = 53607934; // Best if you want to receive all events
let client_build_nubmer = 369195; // You should always use the latest build number
// you can also fetch the build number from the crate `discord_client_rest`

let mut client = GatewayClient::connect(token, capabilities, client_build_nubmer)
    .await
    .unwrap();
```

### Receive events

```rust
loop {
    let event = client.next_event().await.unwrap();
    println!("{}", event);
    // Print the new messages content
    if let Event::MessageCreate(message_create) = event {
        let centent = message_create.message.content.unwrap_or("No content".to_string());
        println!("Message: {}", content);
    }
}
```

### Subscribe to guilds

This example shows how to subscribe to all guilds after receiving the `Ready` event.

You can just subscribe to the guilds you want to receive events from by giving the guild ids to the `bulk_guild_subscribe` method.

```rust
let event = client.next_event().await.unwrap();
println!("{}", event);

if let Event::Ready(ready) = event {
    let mut ids: Vec<u64> = Vec::new();
    let guilds = ready.guilds;
    for guild in guilds {
        let guild_id = guild.id;
        ids.push(guild_id);
    }

    client.bulk_guild_subscribe(ids).await.unwrap();
}
```

## Supported events

- **Gateway**
  - [x] Gateway Payload
  - [x] Gateway Reconnect
  - [x] Heartbeat Ack
  - [x] Sessions Replace
  - [ ] Resumed
  - [ ] Remote Command
  - [ ] Auth Session Change
  - [ ] Authenticator Create
  - [ ] Authenticator Delete
  - [ ] Authenticator Update
  - [ ] OAuth2 Token Revoke
- **Ready**
  - [x] Ready
  - [x] Ready Supplemental
- **Message**
  - [x] Message Create
  - [x] Message Update
  - [x] Message Delete
  - [ ] Message Delete Bulk
  - [x] Message Ack
  - [x] Last Messages
  - **Reactions**
    - [x] Message Reaction Add
    - [x] Message Reaction Remove
    - [x] Message Reaction Add Many
    - [x] Message Reaction Remove Emoji
    - [x] Message Reaction Remove All
  - **Poll**
    - [x] Message Poll Vote Add
    - [x] Message Poll Vote Remove
  - **Mention**
    - [ ] Recent Mention Delete
- **Channel**
  - [x] Channel Create
  - [x] Channel Update
  - [x] Channel Delete
  - [ ] Channel Statuses
  - **Conversation**
    - [x] Conversation Summary Update
  - **Thread**
    - [x] Thread Create
    - [x] Thread Update
    - [x] Thread Delete
    - [x] Thread List Sync
    - [x] Thread Member Update
    - [x] Thread Members Update
  - **Pin**
    - [ ] Channel Pins Update
  - **Recipient**
    - [ ] Channel Recipient Add
    - [ ] Channel Recipient Remove
  - **Call**
    - [x] Call Create
    - [ ] Call Update
    - [ ] Call Delete
  - **Voice**
    - [x] Voice State Update
    - [x] Voice Channel State Update
    - [ ] Voice Server Update
  - **Typing**
    - [x] Typing Start
  - **DM**
    - [ ] DM Settings Upsell Show
  - **Webhook**
    - [ ] Webhooks Update
- **Guild**
  - [x] Guild Create
  - [x] Guild Update
  - [x] Guild Delete
  - **Member**
    - [x] Guild Member Update
    - [x] Guild Member Add
    - [x] Guild Member Remove
    - [x] Guild Members Chunk
  - **Update**
    - [x] Passive Update V2
  - **Log**
    - [x] Guild Audit Log Entry Create
  - **Ban**
    - [ ] Guild Ban Add
    - [ ] Guild Ban Remove
  - **Auto Moderation**
    - [ ] Auto Moderation Rule Create
    - [ ] Auto Moderation Rule Update
    - [ ] Auto Moderation Rule Delete
    - [ ] Auto Moderation Action Execution
    - [ ] Auto Moderation Mention Raid Detection
  - **Emoji**
    - [ ] Guild Emojis Update
  - **Sticker**
    - [ ] Guild Stickers Update
  - **Requests**
    - [ ] Guild Join Request Create
    - [ ] Guild Join Request Delete
    - [ ] Guild Join Request Update
  - **Role**
    - [ ] Guild Role Create
    - [ ] Guild Role Update
    - [ ] Guild Role Delete
  - **Schedule Event**
    - [ ] Guild Schedule Event Create
    - [ ] Guild Schedule Event Update
    - [ ] Guild Schedule Event Delete
    - [ ] Guild Schedule Event User Add
    - [ ] Guild Schedule Event User Remove
  - **Soundboard**
    - [ ] Guild Soundboard Sound Create
    - [ ] Guild Soundboard Sound Update
    - [ ] Guild Soundboard Sound Delete
    - [ ] Soundboard Sounds
  - **Integration**
    - [ ] Guild Integrations Update
    - [ ] Guild Integration Create
    - [ ] Guild Integration Update
    - [ ] Guild Integration Delete
  - **Invite**
    - [ ] Guild Invite Create
    - [ ] Guild Invite Delete
- **Application**
  - [ ] Application Command Permissions Update
- **Friend**
  - [ ] Friend Suggestion Create
  - [ ] Friend Suggestion Delete
- **Presence**
  - [x] Presence Update
- **Relationship**
  - [ ] Relationship Add
  - [ ] Relationship Remove
  - [ ] Relationship Update
- **Game Relationship**
  - [ ] Game Relationship Add
  - [ ] Game Relationship Remove
- **Stage**
  - [ ] Stage Instance Create
  - [ ] Stage Instance Delete
  - [ ] Stage Instance Update
- **Stream**
  - [ ] Stream Create
  - [ ] Stream Delete
  - [ ] Stream Update
  - [ ] Stream Server Update
- **User**
  - [ ] User Application Update
  - [ ] User Application Remove
  - [ ] User Connections Update
  - [ ] User Merge Operation Completed
  - [ ] User Note Update
  - [ ] User Required Action Update
  - [ ] User Guild Settings Update
  - [ ] User Settings Update
- **Misc**
  - [x] Content Inventory Inbox Stale
  - [x] User Settings Proto Update