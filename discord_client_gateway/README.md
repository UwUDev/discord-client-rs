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
- [x] QoS Heartbeat
- [x] Update Time Spent Session ID
- [x] Update Presence
- [x] Update Voice State
- [x] Ping Voice Server
- [x] Create Stream
- [x] Watch Stream
- [x] Set Stream Paused
- [x] Delete Stream
- [x] Ping Stream Server
- [x] Update Lobby Voice States
- [x] Ping Lobby Voice Server
- [x] Request Guild Members
- [x] Request Call Connect
- [x] Update Guild Subscriptions
- [x] Request Forum Unreads
- [x] Remote Command
- [x] Get Deleted Entity IDs Not Matching Hash
- [x] Request Soundboard Sounds
- [x] Create Speed Test
- [x] Delete Speed Test
- [x] Request Last Messages
- [x] Search Recent Members
- [x] Resync Guild Channels
- [x] Request Channel Statuses
- [x] Request Channel Member Count
- [x] Request Channel Info
- [x] Bulk Guild Subscribe *(client helper for Update Guild Subscriptions)*

## Supported receive events

> Only user-account and shared (user + bot) events are listed; bot-only events are omitted.

- **Connection**
  - [x] Hello
  - [x] Heartbeat ACK
  - [x] Reconnect
  - [x] Invalid Session
  - [x] Remote Command
  - [x] Gateway Payload
- **Ready & Session**
  - [x] Ready
  - [x] Ready Supplemental
  - [x] Resumed
  - [x] Rate Limited
  - [x] Activity Invite Create
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
  - [x] Billing Popup Bridge Callback
- **Calls**
  - [x] Call Create
  - [x] Call Update
  - [x] Call Delete
- **Channels**
  - [x] Channel Create
  - [x] Channel Update
  - [x] Channel Delete
  - [x] Channel Sync
  - [x] Channel Update Partial
  - [x] Channel Info
  - [x] Channel Statuses
  - [x] Channel Member Count Update
  - [x] Channel Unread Update
  - [x] Channel Pins Update
  - [x] Channel Pins Ack
  - [x] Channel Recipient Add
  - [x] Channel Recipient Remove
- **Consoles**
  - [x] Console Command Update
- **Misc**
  - [x] Conversation Summary Update
  - [x] Creator Monetization Restrictions Update
  - [x] Deleted Entity IDs
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
  - [x] Embedded Activity Update V2
- **Entitlements**
  - [x] Entitlement Create
  - [x] Entitlement Update
  - [x] Entitlement Delete
- **Experiments**
  - [x] Experiment Session Override Create
  - [x] Experiment Session Override Delete
- **Friend Suggestions**
  - [x] Friend Suggestion Create
  - [x] Friend Suggestion Delete
- **Game Servers**
  - [x] Game Server Create
  - [x] Game Server Update
  - [x] Game Server Delete
- **Gift Codes**
  - [x] Gift Code Create
  - [x] Gift Code Update
- **Guilds**
  - [x] Guild Create
  - [x] Guild Update
  - [x] Guild Delete
  - [x] Guild Application Command Index Update
  - [x] Guild Applied Boosts Update
  - [x] Guild Audit Log Entry Create
  - [x] Guild Ban Add
  - [x] Guild Ban Remove
  - [x] Guild Directory Entry Create
  - [x] Guild Directory Entry Update
  - [x] Guild Directory Entry Delete
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
  - [x] Guild Official Game Applications Update
  - [x] Guild Powerup Entitlements Create
  - [x] Guild Powerup Entitlements Delete
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
  - [x] Guild Soundboard Sounds Update
  - [x] Soundboard Sounds
- **Integrations**
  - [x] Guild Integrations Update
  - [x] Integration Create
  - [x] Integration Update
  - [x] Integration Delete
- **Interactions**
  - [x] Interaction Create
  - [x] Interaction Failure
  - [x] Interaction Success
  - [x] Application Command Autocomplete Response
  - [x] Interaction Modal Create
  - [x] Interaction IFrame Modal Create
  - [x] Social Layer SKU Purchase Eligibility Response
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
  - [x] Reaction Notification Sent
  - [x] Recent Mention Delete
  - [x] Last Messages
- **Notification Center**
  - [x] Notification Center Item Create
  - [x] Notification Center Item Delete
  - [x] Notification Center Items Ack
  - [x] Notification Center Item Completed
- **Notification Settings**
  - [x] Notification Settings Update
- **OAuth2**
  - [x] OAuth2 Token Create
  - [x] OAuth2 Token Delete
  - [x] OAuth2 Token Revoke
- **Payments**
  - [x] Payment Update
- **Presence**
  - [x] Presence Update
- **Quests**
  - [x] Quests User Status Update
  - [x] Quests User Completion Update
- **Relationships**
  - [x] Relationship Add
  - [x] Relationship Update
  - [x] Relationship Remove
  - [x] Game Relationship Add
  - [x] Game Relationship Remove
- **Game Invites**
  - [x] Game Invite Create
  - [x] Game Invite Delete
  - [x] Game Invite Delete Many
- **Lobbies**
  - [x] Lobby Create
  - [x] Lobby Update
  - [x] Lobby Delete
  - [x] Lobby Member Add
  - [x] Lobby Member Update
  - [x] Lobby Member Remove
  - [x] Lobby Message Create
  - [x] Lobby Message Update
  - [x] Lobby Message Delete
  - [x] Lobby Voice State Update
  - [x] Lobby Voice Server Update
- **Passive Update**
  - [x] Passive Update V1
  - [x] Passive Update V2
- **Saved Messages**
  - [x] Saved Message Create
  - [x] Saved Message Delete
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
  - [x] Speed Test Create
  - [x] Speed Test Server Update
  - [x] Speed Test Update
  - [x] Speed Test Delete
- **Typing**
  - [x] Typing Start
- **Current User**
  - [x] User Update
  - [x] User Application Update
  - [x] User Application Remove
  - [x] User Application Identity Update
  - [x] User Application Identity Remove
  - [x] User Connections Update
  - [x] User Guild Settings Update
  - [x] User Merge Operation Completed
  - [x] User Non Channel Ack
  - [x] User Note Update
  - [x] User Premium Guild Subscription Slot Create
  - [x] User Premium Guild Subscription Slot Update
  - [x] User Premium Guild Subscription Slot Delete
  - [x] User Settings Update
  - [x] Audio Settings Update
  - [x] User Payment Browser Checkout Done
  - [x] User Payment Client Add
  - [x] User Payment Sources Update
  - [x] User Required Action Update
  - [x] User Subscriptions Update
  - [x] User Settings Proto Update
- **Voice**
  - [x] Voice State Update
  - [x] Voice State Update Batch
  - [x] Voice Server Update
  - [x] Voice Channel Effect Send
  - [x] Voice Channel Start Time Update
  - [x] Voice Channel Status Update
- **Virtual Currency**
  - [x] Virtual Currency Balance Update
- **Webhooks**
  - [x] Webhooks Update
