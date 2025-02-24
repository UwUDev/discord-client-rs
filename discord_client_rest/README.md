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

## Supported interactions

- **Applications**
  - [ ] Get Applications
  - [ ] Get Applications with Assets
  - [ ] Create Application
  - [ ] Get Application
  - [ ] Get Current Application
  - [ ] Modify Application
  - [ ] Modify Current Application
  - [ ] Delete Application
  - [ ] Transfer Application
  - [ ] Reset Application Secret
  - [ ] Create Application Bot
  - [ ] Modify Application Bot
  - [ ] Reset Application Bot Token
  - [ ] Get Application Discoverability State
  - [ ] Get Embedded Activities
  - [ ] Set Application Embeddability
  - [ ] Get Application Embedded Activity Config
  - [ ] Modify Application Embedded Activity Config
  - [ ] Get Application Proxy Config
  - [ ] Modify Application Proxy Config
  - [ ] Get Application Assets
  - [ ] Create Application Asset
  - [ ] Delete Application Asset
  - [ ] Create Application Attachment
  - [ ] Get Detectable Applications
  - [ ] Get Partial Applications
  - [ ] Get Partial Application
  - [ ] Get Rich Presence Application
  - [ ] Get Application Disclosures
  - [ ] Acknowledge Application Disclosures
  - [ ] Get Guild Applications
  - [ ] Report Unverified Application
  - [ ] Upload Unverified Application Icon
  - [ ] Get User Application Role Connection
  - [ ] Modify User Application Role Connection
- **Audit loc**
  - [ ] Get Guild Audit Log
- **Auto moderation**
  - [ ] Get Guild AutoMod Rules
  - [ ] Get Guild AutoMod Rule
  - [ ] Create Guild AutoMod Rule
  - [ ] Validate Guild AutoMod Rule
  - [ ] Modify Guild AutoMod Rule
  - [ ] Delete Guild AutoMod Rule
  - [ ] Execute AutoMod Alert Action
  - [ ] Modify AutoMod Incident Actions
  - [ ] Resolve AutoMod Incident
  - [ ] Report AutoMod Incident
  - [ ] Clear Mention Raid Incident
- **Channels**
  - [x] Get Private Channels
  - [ ] Get DM Channel
  - [x] Create Private Channel
  - [ ] Get Guild Channels
  - [ ] Get Guild Top Read Channels
  - [x] Create Guild Channel
  - [ ] Modify Guild Channel Positions
  - [ ] Get Channel
  - [x] Modify Channel
  - [x] Delete Channel
  - [ ] Delete Read State
  - [ ] Modify Channel Status
  - [ ] Modify Channel Permissions
  - [ ] Delete Channel Permission
  - [ ] Follow Channel
  - [ ] Trigger Typing Indicator
  - [ ] Get Call Eligibility
  - [ ] Modify Call
  - [ ] Ring Channel Recipients
  - [ ] Stop Ringing Channel Recipients
  - [x] Add Channel Recipient
  - [x] Remove Channel Recipient
  - [ ] Update Message Request
  - [ ] Reject Message Request
  - [ ] Batch Reject Message Requests
  - [ ] Get Supplemental Message Request Data
  - [ ] Acknowledge Blocked User Warning
  - [ ] Acknowledge Safety Warnings
  - [ ] Add Safety Warning
  - [ ] Delete Safety Warnings
  - [ ] Report Safety Warning False Positive
  - [ ] Get Guild Active Threads
  - [ ] Get Active Threads
  - [ ] Get Public Archived Threads
  - [ ] Get Private Archived Threads
  - [ ] Get Joined Private Archived Threads
  - [ ] Search Threads
  - [ ] Create Thread from Message
  - [ ] Create Thread
  - [ ] Get Channel Post Data
  - [ ] Get Thread Members
  - [ ] Get Thread Member
  - [ ] Join Thread
  - [ ] Add Thread Member
  - [ ] Modify Thread Settings
  - [ ] Leave Thread
  - [ ] Remove Thread Member
  - [ ] Create Channel Tag
  - [ ] Modify Channel Tag
  - [ ] Delete Channel Tag
- **Clan**
  - [ ] Set Clan Identity
  - [ ] Get Clan
  - [ ] Create Clan
  - [ ] Get Clan Settings
  - [ ] Modify Clan Settings
  - [ ] Disable Clan
- **Discovery**
  - [ ] Get Discoverable Guilds
  - [ ] Search Discoverable Guilds
  - [ ] Search Published Guilds
  - [ ] Get Discovery Slug
  - [ ] Get Discovery Categories
  - [ ] Validate Discovery Search Term
  - [ ] Get Guild Discovery Requirements
  - [ ] Get Guild Discovery Metadata
  - [ ] Modify Guild Discovery Metadata
  - [ ] Add Guild Discovery Subcategory
  - [ ] Remove Guild Discovery Subcategory
- **Emoji**
  - [ ] Get Guild Emojis
  - [ ] Get Guild Emoji
  - [ ] Get Guild Top Emojis
  - [ ] Get Emoji Guild
  - [ ] Get Emoji Source
  - [ ] Create Guild Emoji
  - [ ] Modify Guild Emoji
  - [ ] Delete Guild Emoji
  - [ ] Get Application Emojis
  - [ ] Get Application Emoji
  - [ ] Create Application Emoji
  - [ ] Modify Application Emoji
  - [ ] Delete Application Emoji
- **Family center**
  - [ ] Get Family Center Overview
  - [ ] Get Link Code
  - [ ] Get Linked Users
  - [ ] Create Linked User Request
  - [ ] Modify Linked User
- **Guild**
  - [ ] Get User Guilds
  - [ ] Get Join Request Guilds
  - [ ] Leave Guild
  - [ ] Create Guild
  - [ ] Get Guild
  - [ ] Get Guild Basic
  - [ ] Get Guild Preview
  - [ ] Modify Guild
  - [ ] Modify Guild MFA Level
  - [ ] Delete Guild
  - [ ] Get Guild Members
  - [ ] Query Guild Members
  - [ ] Search Guild Members
  - [ ] Get Guild Members Supplemental
  - [ ] Get Guild Member
  - [ ] Join Guild
  - [ ] Add Guild Member
  - [ ] Modify Guild Member
  - [ ] Modify Current Guild Member
  - [ ] Modify Current Guild Member Nick
  - [ ] Modify Guild Member Profile
  - [ ] Add Guild Member Role
  - [ ] Remove Guild Member Role
  - [ ] Remove Guild Member
  - [ ] Acknowledge DM Settings Upsell Modal
  - [ ] Get Guild Bans
  - [ ] Search Guild Bans
  - [ ] Get Guild Ban
  - [ ] Create Guild Ban
  - [ ] Bulk Guild Ban
  - [ ] Delete Guild Ban
  - [ ] Get Guild Roles
  - [ ] Get Guild Role
  - [ ] Get Guild Role Member Counts
  - [ ] Get Guild Role Members
  - [ ] Add Guild Role Members
  - [ ] Create Guild Role
  - [ ] Modify Guild Role Positions
  - [ ] Modify Guild Role
  - [ ] Delete Guild Role
  - [ ] Get Guild Prune
  - [ ] Prune Guild
  - [ ] Get Guild Widget Settings
  - [ ] Modify Guild Widget
  - [ ] Get Guild Widget
  - [ ] Get Guild Widget Image
  - [ ] Get Guild Vanity Invite
  - [ ] Modify Guild Vanity Invite
  - [ ] Get Guild Member Verification
  - [ ] Modify Guild Member Verification
  - [ ] Get Guild Join Requests
  - [ ] Get Guild Join Request
  - [ ] Get Guild Join Request Cooldown
  - [ ] Create Guild Join Request
  - [ ] Reset Guild Join Request
  - [ ] Delete Guild Join Request
  - [ ] Create Guild Join Request Interview
  - [ ] Action Guild Join Request
  - [ ] Action Guild Join Request by User
  - [ ] Bulk Action Guild Join Requests
  - [ ] Get Guild Welcome Screen
  - [ ] Modify Guild Welcome Screen
  - [ ] Get Guild Onboarding
  - [ ] Modify Guild Onboarding
  - [ ] Get Admin Community Eligibility
  - [ ] Join Admin Community
  - [ ] Join Wumpus Feedback Squad
- **Guild scheduled events**
  - [ ] Get User Guild Scheduled Events
  - [ ] Get Guild Scheduled Events
  - [ ] Get Guild Scheduled Event
  - [ ] Create Guild Scheduled Event
  - [ ] Modify Guild Scheduled Event
  - [ ] Delete Guild Scheduled Event
  - [ ] Create Guild Scheduled Event Exception
  - [ ] Modify Guild Scheduled Event Exception
  - [ ] Delete Guild Scheduled Event Exception
  - [ ] Get Guild Scheduled Event User Count
  - [ ] Get Guild Scheduled Event Users
  - [ ] Create Guild Scheduled Event User
  - [ ] Delete Guild Scheduled Event User
  - [ ] Get Guild Scheduled Event Exception Users
  - [ ] Create Guild Scheduled Event Exception User
  - [ ] Delete Guild Scheduled Event Exception User
- **Guild template**
  - [ ] Get Guild Template
  - [ ] Use Guild Template
  - [ ] Get Guild Templates
  - [ ] Create Guild Template
  - [ ] Sync Guild Template
  - [ ] Modify Guild Template
  - [ ] Delete Guild Template
- **Integration**
  - [ ] Get Guild Integrations
  - [ ] Enable Guild Integration
  - [ ] Sync Guild Integration
  - [ ] Modify Guild Integration
  - [ ] Delete Guild Integration
  - [ ] Migrate Guild Command Scope
  - [ ] Get Guild Integration Application IDs
  - [ ] Get Channel Integrations
  - [ ] Delete Channel Integration
  - [ ] Join Integration Guild
  - [ ] Search Tenor GIFs
  - [ ] Get Trending GIF Search Terms
  - [ ] Get Suggested GIF Search Terms
  - [ ] Search GIFs
  - [ ] Get Trending GIF Categories
  - [ ] Get Trending GIFs
  - [ ] Track Selected GIF
- **Invite**
  - [ ] Get Invite
  - [ ] Accept Invite
  - [x] Delete Invite
  - [x] Get Guild Invites
  - [x] Get Channel Invites
  - [x] Create Channel Invite
  - [ ] Get User Invites
  - [ ] Create User Invite
  - [ ] Revoke User Invites
- **Message**
  - [x] Get Messages
  - [ ] Preload Messages
  - [x] Search Messages
  - [x] Get Message
  - [x] Create Message
  - [x] Create DM Message
  - [ ] Create Greet Message
  - [ ] Create Attachments
  - [ ] Delete Attachment
  - [ ] Refresh Attachment URLs
  - [ ] Acknowledge Message
  - [ ] Crosspost Message
  - [ ] Hide Message from Guild Feed
  - [ ] Get Reactions
  - [ ] Create Reaction
  - [ ] Delete Own Reaction
  - [ ] Delete Reaction
  - [ ] Delete Reaction Emoji
  - [ ] Delete All Reactions
  - [x] Edit Message
  - [x] Edit DM Message
  - [x] Delete Message
  - [x] Delete DM Message
  - [x] Get Pinned Messages
  - [x] Pin Message
  - [x] Unpin Message
  - [ ] Acknowledge Pinned Messages
  - [ ] Get Channel Media Preview
  - [ ] Unfurl Embed
  - [ ] Unfurl Embeds
  - [ ] Get Answer Voters
  - [ ] End Poll
  - [ ] Create Poll Vote
  - [ ] Get Conversation Summaries
  - [ ] Delete Conversation Summary
- **Premium referrals**
  - [ ] Get Premium Referral
  - [ ] Get Premium Referral Eligibility
  - [ ] Get Premium Referral Eligible Users
  - [ ] Preview Premium Referral
  - [ ] Create Premium Referral
- **Presence**
  - [ ] Get Presences
  - [ ] Update Presence
  - [ ] Create Headless Session
  - [ ] Delete Headless Session
  - [ ] Get Activity Metadata
  - [ ] Get Activity Secret
- **Quest**
  - [ ] Get Available Quests
  - [ ] Accept Quest
  - [ ] Claim Quest Reward
  - [ ] Get Quest Reward Code
  - [ ] Send Quest Heartbeat
  - [ ] Complete Quest
  - [ ] Reset Quest
  - [ ] Dismiss Quest Content
  - [ ] Reset Quest Dismissibility
- **Relationship**
  - [ ] Get Relationships
  - [ ] Send Friend Request
  - [ ] Create Relationship
  - [ ] Ignore User
  - [ ] Unignore User
  - [ ] Modify Relationship
  - [ ] Remove Relationship
  - [ ] Bulk Remove Relationships
  - [ ] Get Game Relationships
  - [ ] Send Game Friend Request
  - [ ] Create Game Relationship
  - [ ] Create Game Relationship by Application
  - [ ] Remove Game Relationship
  - [ ] Remove Game Relationship by Application
  - [ ] Get Friend Suggestions
  - [ ] Remove Friend Suggestion
- **Soundboard**
  - [ ] Get Default Soundboard Sounds
  - [ ] Get Guild Soundboard Sounds
  - [ ] Get Guild Soundboard Sound
  - [ ] Create Guild Soundboard Sound
  - [ ] Modify Guild Soundboard Sound
  - [ ] Delete Guild Soundboard Sound
  - [ ] Get Soundboard Sound Guild
  - [ ] Send Soundboard Sound
- **Stage instance**
  - [ ] Get Stage Instance
  - [ ] Modify Stage Instance
  - [ ] Delete Stage Instance
- **Sticker**
  - [ ] Get Sticker Packs
  - [ ] Get Sticker Pack
  - [ ] Get Sticker
  - [ ] Get Sticker Guild
  - [ ] Get Guild Stickers
  - [ ] Get Guild Sticker
  - [ ] Create Guild Sticker
  - [ ] Modify Guild Sticker
  - [ ] Delete Guild Sticker
- **Team**
  - [ ] Get Teams
  - [ ] Create Team
  - [ ] Get Team
  - [ ] Modify Team
  - [ ] Delete Team
  - [ ] Accept Team Invite
  - [ ] Get Team Members
  - [ ] Add Team Member
  - [ ] Modify Team Member
  - [ ] Remove Team Member
  - [ ] Get Team Applications
  - [ ] Get Team Stripe Connect URL
  - [ ] Get Team Payout Onboarding
  - [ ] Get Team Payouts
  - [ ] Get Team Payout Report
  - [ ] Search Companies
  - [ ] Create Company
- **User**
  - [ ] Get Current User
  - [ ] Modify Current User
  - [ ] Modify Current User Account
  - [ ] Get User
  - [ ] Get User Profile
  - [ ] Modify User Profile
  - [ ] Get Mutual Relationships
  - [ ] Enable TOTP MFA
  - [ ] Disable TOTP MFA
  - [ ] Enable SMS MFA
  - [ ] Disable SMS MFA
  - [ ] Get WebAuthn Authenticators
  - [ ] Create WebAuthn Authenticator
  - [ ] Modify WebAuthn Authenticator
  - [ ] Delete WebAuthn Authenticator
  - [ ] Send Backup Codes Challenge
  - [ ] Get Backup Codes
  - [ ] Disable User
  - [ ] Delete User
  - [ ] Verify User Captcha
  - [ ] Modify User Email
  - [ ] Verify User Email Change
  - [ ] Get Pomelo Suggestions
  - [ ] Get Pomelo Eligibility
  - [ ] Create Pomelo Migration
  - [ ] Get Recent Mentions
  - [ ] Delete Recent Mention
  - [ ] Get User Harvest
  - [ ] Create User Harvest
  - [ ] Get User Notes
  - [ ] Get User Note
  - [ ] Modify User Note
  - [ ] Authorize User Connection
  - [ ] Create User Connection Callback
  - [ ] Create Contact Sync Connection
  - [ ] Create Domain Connection
  - [ ] Get User Connections
  - [ ] Get User Connection Access Token
  - [ ] Get User Connection Subreddits
  - [ ] Refresh User Connection
  - [ ] Modify User Connection
  - [ ] Delete User Connection
  - [ ] Get Friend Token
  - [ ] Get User Affinities
  - [ ] Get User Affinities v2
  - [ ] Get Guild Affinities
  - [ ] Get Channel Affinities
  - [ ] Confirm Tutorial Indicator
  - [ ] Suppress Tutorial
  - [ ] Join HypeSquad Online
  - [ ] Leave HypeSquad Online
  - [ ] Join Active Developer Program
  - [ ] Leave Active Developer Program
  - [ ] Get User Premium Usage
  - [ ] Get User Profile Effects
  - [ ] Get Confetti Potions
  - [ ] Apply Confetti Potion
- **User settings**
  - [ ] Get User Settings Proto
  - [ ] Modify User Settings Proto
  - [ ] Get User Settings
  - [ ] Modify User Settings
  - [ ] Get User Consents
  - [ ] Modify User Consents
  - [ ] Get Email Settings
  - [ ] Modify Email Settings
  - [ ] Modify Notification Settings
  - [ ] Get Notification Settings Snapshots
  - [ ] Create Notification Settings Snapshot
  - [ ] Restore Notification Settings Snapshot
  - [ ] Delete Notification Settings Snapshot
  - [ ] Modify User Guild Settings
  - [ ] Bulk Modify User Guild Settings
- **Voice**
  - [ ] Get Voice Regions
  - [ ] Get Guild Voice Regions
  - [ ] Get Voice Filters Catalog
  - [ ] Get Current User Voice State
  - [ ] Get User Voice State
  - [ ] Modify Current User Voice State
  - [ ] Modify User Voice State
  - [ ] Send Voice Channel Effect
  - [ ] Get Stream Preview
  - [ ] Upload Stream Preview
  - [ ] Broadcast Stream Notification
- **Webhook**
  - [ ] Create Webhook
  - [ ] Get Channel Webhooks
  - [ ] Get Guild Webhooks
  - [ ] Get Webhook
  - [ ] Get Webhook with Token
  - [ ] Modify Webhook
  - [ ] Modify Webhook with Token
  - [ ] Delete Webhook
  - [ ] Delete Webhook with Token
  - [ ] Execute Webhook
  - [ ] Execute Slack-Compatible Webhook
  - [ ] Execute GitHub-Compatible Webhook
  - [ ] Get Webhook Message
  - [ ] Edit Webhook Message
  - [ ] Delete Webhook Message
