# Discord Client Gateway

A high-level Rust implementation of the Discord gateway, designed to provide a robust and efficient client-side connection to Discord's real-time WebSocket API.

This crate offers a seamless integration for Discord bot developers, featuring:

- **Zlib-stream decompression** support for Discord's compressed payloads
- **TLS impersonation** and **HTTP/2 (H2) mimicing** of Chrome to avoid detection
- Automatic heartbeat with sequence number handling
- Efficient handling of Discord's WebSocket events
- Automatic shard management for scalable bot implementations
- Reconnect and resume support

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
## Supported send events

- [x] Identify
- [X] Resume
- [x] Heartbeat
- [ ] Update Presence
- [ ] Update Voice State
- [ ] Ping Voice Server
- [ ] Create Stream
- [ ] Watch Stream
- [ ] Set Stream Paused
- [ ] Delete Stream
- [ ] Ping Stream Server
- [x] Bulk guild subscribe
- [x] Request Guild Members
- [ ] Request Call Connect
- [ ] Update Guild Subscriptions
- [ ] Request Forum Unreads
- [ ] Remote Command
- [ ] Request Deleted Entity IDs
- [ ] Request Soundboard Sounds
- [x] Request Last Messages
- [x] Search Recent Members
- [x] Request Channel Statuses

## Supported receive events

- **Application**
  - [ ] Application Command Permissions Update
- **Channel**
  - **Call**
    - [x] Call Create
    - [x] Call Delete
    - [x] Call Update
  - **Conversation**
    - [x] Conversation Summary Update
  - **DM**
    - [ ] DM Settings Upsell Show
  - **Pin**
    - [x] Channel Pins Update
    - [x] Channel Pins ACX
  - **Recipient**
    - [x] Channel Recipient Add
    - [x] Channel Recipient Remove
  - **Thread**
    - [x] Thread Create
    - [x] Thread Delete
    - [x] Thread List Sync
    - [x] Thread Member Update
    - [x] Thread Members Update
    - [x] Thread Update
  - **Typing**
    - [x] Typing Start
  - **Voice**
    - [x] Voice Channel State Update
    - [ ] Voice Server Update
    - [x] Voice State Update
  - **Webhook**
    - [ ] Webhooks Update
  - [x] Channel Create
  - [x] Channel Delete
  - [x] Channel Statuses
  - [x] Channel Update
- **Friend**
  - [ ] Friend Suggestion Create
  - [ ] Friend Suggestion Delete
- **Game Relationship**
  - [ ] Game Relationship Add
  - [ ] Game Relationship Remove
- **Gateway**
  - [ ] Auth Session Change
  - [ ] Authenticator Create
  - [ ] Authenticator Delete
  - [ ] Authenticator Update
  - [x] Gateway Payload
  - [x] Gateway Reconnect
  - [x] Heartbeat Ack
  - [ ] OAuth2 Token Revoke
  - [ ] Remote Command
  - [x] Resumed
  - [x] Sessions Replace
- **Guild**
  - **ACK**
    - [x] Guild Features Ack
  - **Auto Moderation**
    - [x] Auto Moderation Mention Raid Detection
  - **Ban**
    - [x] Guild Ban Add
    - [x] Guild Ban Remove
  - **Channels**
    - [x] Channels unread update
  - **Emoji**
    - [x] Guild Emojis Update
  - **Integration**
    - [x] Guild Integration Create
    - [x] Guild Integration Delete
    - [x] Guild Integration Update
    - [x] Guild Integrations Update
  - **Log**
    - [x] Guild Audit Log Entry Create
  - **Member**
    - [x] Guild Member Add
    - [x] Guild Member Remove
    - [x] Guild Member Update
    - [x] Guild Members Chunk
  - **Requests**
    - [ ] Guild Join Request Create
    - [ ] Guild Join Request Delete
    - [ ] Guild Join Request Update
  - **Role**
    - [x] Guild Role Create
    - [x] Guild Role Delete
    - [x] Guild Role Update
  - **Schedule Event**
    - [x] Guild Schedule Event Create
    - [x] Guild Schedule Event Delete
    - [x] Guild Schedule Event Update
    - [x] Guild Schedule Event User Add
    - [x] Guild Schedule Event User Remove
    - [x] Guild Scheduled Event Exception Create
    - [x] Guild Scheduled Event Exception Delete
    - [x] Guild Scheduled Event Exceptions Delete
    - [x] Guild Scheduled Event Exception Update
  - **Soundboard**
    - [ ] Guild Soundboard Sound Create
    - [ ] Guild Soundboard Sound Delete
    - [ ] Guild Soundboard Sound Update
    - [ ] Soundboard Sounds
  - **Sticker**
    - [x] Guild Stickers Update
  - **Update**
    - [x] Passive Update V2
  - [x] Guild Create
  - [x] Guild Delete
  - [x] Guild Update
- **Message**
  - **Mention**
    - [ ] Recent Mention Delete
  - **Poll**
    - [x] Message Poll Vote Add
    - [x] Message Poll Vote Remove
  - **Reactions**
    - [x] Message Reaction Add
    - [x] Message Reaction Add Many
    - [x] Message Reaction Remove
    - [x] Message Reaction Remove All
    - [x] Message Reaction Remove Emoji
  - [x] Last Messages
  - [x] Message Ack
  - [x] Message Create
  - [x] Message Delete
  - [x] Message Delete Bulk
  - [x] Message Update
- **Misc**
  - [x] Content Inventory Inbox Stale
  - [x] User Settings Proto Update
- **Presence**
  - [x] Presence Update
- **Ready**
  - [x] Ready
  - [x] Ready Supplemental
- **Stage**
  - [ ] Stage Instance Create
  - [ ] Stage Instance Delete
  - [ ] Stage Instance Update
- **Stream**
  - [ ] Stream Create
  - [ ] Stream Delete
  - [ ] Stream Server Update
  - [ ] Stream Update
- **User**
  - [ ] User Application Remove
  - [ ] User Application Update
  - [ ] User Connections Update
  - [ ] User Guild Settings Update
  - [ ] User Merge Operation Completed
  - [ ] User Note Update
  - [ ] User Required Action Update
  - [ ] User Settings Update
  - **Relationship**
    - [ ] Relationship Add
    - [ ] Relationship Remove
    - [ ] Relationship Update