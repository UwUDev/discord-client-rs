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
let client_build_nubmer = 402402; // You should always use the latest build number
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
- [x] Resume
- [x] Heartbeat
- [ ] QoS Heartbeat
- [ ] Update Time Spent Session ID
- [x] Update Presence
- [x] Update Voice State
- [ ] Ping Voice Server
- [x] Create Stream
- [x] Watch Stream
- [x] Set Stream Paused
- [x] Delete Stream
- [x] Ping Stream Server
- [ ] Update Lobby Voice States
- [ ] Ping Lobby Voice Server
- [x] Request Guild Members
- [ ] Request Call Connect
- [x] Update Guild Subscriptions
- [ ] Request Forum Unreads
- [x] Remote Command
- [ ] Get Deleted Entity IDs Not Matching Hash
- [x] Request Soundboard Sounds
- [ ] Create Speed Test
- [ ] Delete Speed Test
- [x] Request Last Messages
- [x] Search Recent Members
- [ ] Resync Guild Channels
- [x] Request Channel Statuses
- [ ] Request Channel Member Count
- [ ] Request Channel Info
- [x] Bulk Guild Subscribe *(client helper for Update Guild Subscriptions)*

## Supported receive events

> Only user-account and shared (user + bot) events are listed; bot-only events are omitted.

- **Connection**
  - [ ] Hello
  - [x] Heartbeat ACK
  - [x] Reconnect
  - [ ] Invalid Session
  - [x] Remote Command
  - [x] Gateway Payload
- **Ready & Session**
  - [x] Ready
  - [x] Ready Supplemental
  - [x] Resumed
  - [ ] Rate Limited
  - [ ] Activity Invite Create
- **Authentication**
  - [x] Auth Session Change
  - [x] Authenticator Create
  - [x] Authenticator Update
  - [x] Authenticator Delete
- **Application Commands**
  - [x] Application Command Permissions Update
- **Auto Moderation**
  - [x] Auto Moderation Mention Raid Detection
- **Billing**
  - [ ] Billing Popup Bridge Callback
- **Calls**
  - [x] Call Create
  - [x] Call Update
  - [x] Call Delete
- **Channels**
  - [x] Channel Create
  - [x] Channel Update
  - [x] Channel Delete
  - [ ] Channel Sync
  - [ ] Channel Update Partial
  - [ ] Channel Info
  - [x] Channel Statuses
  - [ ] Channel Member Count Update
  - [x] Channel Unread Update
  - [x] Channel Pins Update
  - [x] Channel Pins Ack
  - [x] Channel Recipient Add
  - [x] Channel Recipient Remove
- **Consoles**
  - [ ] Console Command Update
- **Misc**
  - [x] Conversation Summary Update
  - [ ] Creator Monetization Restrictions Update
  - [ ] Deleted Entity IDs
  - [x] DM Settings Upsell Show
  - [x] Content Inventory Inbox Stale
  - [x] Generic Push Notification Sent
- **Threads**
  - [x] Thread Create
  - [x] Thread Update
  - [x] Thread Delete
  - [x] Thread List Sync
  - [x] Thread Member Update
  - [x] Thread Members Update
- **Embedded Activities**
  - [ ] Embedded Activity Update V2
- **Entitlements**
  - [ ] Entitlement Create
  - [ ] Entitlement Update
  - [ ] Entitlement Delete
- **Experiments**
  - [ ] Experiment Session Override Create
  - [ ] Experiment Session Override Delete
- **Friend Suggestions**
  - [x] Friend Suggestion Create
  - [x] Friend Suggestion Delete
- **Game Servers**
  - [ ] Game Server Create
  - [ ] Game Server Update
  - [ ] Game Server Delete
- **Gift Codes**
  - [ ] Gift Code Create
  - [ ] Gift Code Update
- **Guilds**
  - [x] Guild Create
  - [x] Guild Update
  - [x] Guild Delete
  - [ ] Guild Application Command Index Update
  - [x] Guild Applied Boosts Update
  - [x] Guild Audit Log Entry Create
  - [x] Guild Ban Add
  - [x] Guild Ban Remove
  - [ ] Guild Directory Entry Create
  - [ ] Guild Directory Entry Update
  - [ ] Guild Directory Entry Delete
  - [x] Guild Emojis Update
  - [x] Guild Stickers Update
  - [x] Guild Feature Ack
  - [x] Guild Join Request Create
  - [x] Guild Join Request Update
  - [x] Guild Join Request Delete
  - [x] Guild Member Add
  - [x] Guild Member Update
  - [x] Guild Member Remove
  - [x] Guild Members Chunk
  - [ ] Guild Official Game Applications Update
  - [ ] Guild Powerup Entitlements Create
  - [ ] Guild Powerup Entitlements Delete
  - [x] Guild Role Create
  - [x] Guild Role Update
  - [x] Guild Role Delete
- **Guild Scheduled Events**
  - [x] Guild Scheduled Event Create
  - [x] Guild Scheduled Event Update
  - [x] Guild Scheduled Event Delete
  - [x] Guild Scheduled Event Exception Create
  - [x] Guild Scheduled Event Exception Update
  - [x] Guild Scheduled Event Exception Delete
  - [x] Guild Scheduled Event Exceptions Delete
  - [x] Guild Scheduled Event User Add
  - [x] Guild Scheduled Event User Remove
- **Guild Soundboard**
  - [x] Guild Soundboard Sound Create
  - [x] Guild Soundboard Sound Update
  - [x] Guild Soundboard Sound Delete
  - [ ] Guild Soundboard Sounds Update
  - [x] Soundboard Sounds
- **Integrations**
  - [x] Guild Integrations Update
  - [x] Integration Create
  - [x] Integration Update
  - [x] Integration Delete
- **Interactions**
  - [ ] Interaction Create
  - [ ] Interaction Failure
  - [ ] Interaction Success
  - [ ] Application Command Autocomplete Response
  - [ ] Interaction Modal Create
  - [ ] Interaction IFrame Modal Create
  - [ ] Social Layer SKU Purchase Eligibility Response
- **Messages**
  - [x] Message Create
  - [x] Message Update
  - [x] Message Delete
  - [x] Message Delete Bulk
  - [x] Message Ack
  - [x] Message Poll Vote Add
  - [x] Message Poll Vote Remove
  - [x] Message Reaction Add
  - [x] Message Reaction Add Many
  - [x] Message Reaction Remove
  - [x] Message Reaction Remove All
  - [x] Message Reaction Remove Emoji
  - [ ] Reaction Notification Sent
  - [x] Recent Mention Delete
  - [x] Last Messages
- **Notification Center**
  - [ ] Notification Center Item Create
  - [ ] Notification Center Item Delete
  - [ ] Notification Center Items Ack
  - [ ] Notification Center Item Completed
- **Notification Settings**
  - [ ] Notification Settings Update
- **OAuth2**
  - [ ] OAuth2 Token Create
  - [ ] OAuth2 Token Delete
  - [x] OAuth2 Token Revoke
- **Payments**
  - [ ] Payment Update
- **Presence**
  - [x] Presence Update
- **Quests**
  - [ ] Quests User Status Update
  - [ ] Quests User Completion Update
- **Relationships**
  - [x] Relationship Add
  - [x] Relationship Update
  - [x] Relationship Remove
  - [x] Game Relationship Add
  - [x] Game Relationship Remove
- **Game Invites**
  - [ ] Game Invite Create
  - [ ] Game Invite Delete
  - [ ] Game Invite Delete Many
- **Lobbies**
  - [ ] Lobby Create
  - [ ] Lobby Update
  - [ ] Lobby Delete
  - [ ] Lobby Member Add
  - [ ] Lobby Member Update
  - [ ] Lobby Member Remove
  - [ ] Lobby Message Create
  - [ ] Lobby Message Update
  - [ ] Lobby Message Delete
  - [ ] Lobby Voice State Update
  - [ ] Lobby Voice Server Update
- **Passive Update**
  - [ ] Passive Update V1
  - [x] Passive Update V2
- **Saved Messages**
  - [ ] Saved Message Create
  - [ ] Saved Message Delete
- **Sessions**
  - [x] Sessions Replace
- **Stage Instances**
  - [x] Stage Instance Create
  - [x] Stage Instance Update
  - [x] Stage Instance Delete
- **Streams**
  - [x] Stream Create
  - [x] Stream Server Update
  - [x] Stream Update
  - [x] Stream Delete
  - [ ] Speed Test Create
  - [ ] Speed Test Server Update
  - [ ] Speed Test Update
  - [ ] Speed Test Delete
- **Typing**
  - [x] Typing Start
- **Current User**
  - [ ] User Update
  - [x] User Application Update
  - [x] User Application Remove
  - [ ] User Application Identity Update
  - [ ] User Application Identity Remove
  - [x] User Connections Update
  - [x] User Guild Settings Update
  - [x] User Merge Operation Completed
  - [ ] User Non Channel Ack
  - [x] User Note Update
  - [ ] User Premium Guild Subscription Slot Create
  - [ ] User Premium Guild Subscription Slot Update
  - [ ] User Premium Guild Subscription Slot Delete
  - [x] User Settings Update
  - [ ] Audio Settings Update
  - [ ] User Payment Browser Checkout Done
  - [ ] User Payment Client Add
  - [ ] User Payment Sources Update
  - [x] User Required Action Update
  - [ ] User Subscriptions Update
  - [x] User Settings Proto Update
- **Voice**
  - [x] Voice State Update
  - [x] Voice Server Update
  - [ ] Voice Channel Effect Send
  - [x] Voice Channel Start Time Update
  - [x] Voice Channel Status Update
- **Virtual Currency**
  - [ ] Virtual Currency Balance Update
- **Webhooks**
  - [x] Webhooks Update
