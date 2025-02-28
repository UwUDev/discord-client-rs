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
  - [ ] Acknowledge Application Disclosures
  - [ ] Create Application
  - [ ] Create Application Asset
  - [ ] Create Application Attachment
  - [ ] Create Application Bot
  - [ ] Delete Application
  - [ ] Delete Application Asset
  - [ ] Get Application
  - [ ] Get Application Assets
  - [ ] Get Application Disclosures
  - [ ] Get Application Discoverability State
  - [ ] Get Application Embedded Activity Config
  - [ ] Get Application Proxy Config
  - [ ] Get Applications
  - [ ] Get Applications with Assets
  - [ ] Get Current Application
  - [ ] Get Detectable Applications
  - [ ] Get Embedded Activities
  - [ ] Get Guild Applications
  - [ ] Get Partial Application
  - [ ] Get Partial Applications
  - [ ] Get Rich Presence Application
  - [ ] Get User Application Role Connection
  - [ ] Modify Application
  - [ ] Modify Application Bot
  - [ ] Modify Application Embedded Activity Config
  - [ ] Modify Application Proxy Config
  - [ ] Modify Current Application
  - [ ] Modify User Application Role Connection
  - [ ] Report Unverified Application
  - [ ] Reset Application Bot Token
  - [ ] Reset Application Secret
  - [ ] Set Application Embeddability
  - [ ] Transfer Application
  - [ ] Upload Unverified Application Icon
- **Audit loc**
  - [x] Get Guild Audit Log
- **Auto moderation**
  - [ ] Clear Mention Raid Incident
  - [ ] Create Guild AutoMod Rule
  - [ ] Delete Guild AutoMod Rule
  - [ ] Execute AutoMod Alert Action
  - [ ] Get Guild AutoMod Rule
  - [ ] Get Guild AutoMod Rules
  - [ ] Modify AutoMod Incident Actions
  - [ ] Modify Guild AutoMod Rule
  - [ ] Report AutoMod Incident
  - [ ] Resolve AutoMod Incident
  - [ ] Validate Guild AutoMod Rule
- **Channels**
  - [ ] Acknowledge Blocked User Warning
  - [ ] Acknowledge Safety Warnings
  - [x] Add Channel Recipient
  - [ ] Add Safety Warning
  - [ ] Add Thread Member
  - [ ] Batch Reject Message Requests
  - [ ] Create Channel Tag
  - [x] Create Guild Channel
  - [x] Create Private Channel
  - [ ] Create Thread
  - [ ] Create Thread from Message
  - [x] Delete Channel
  - [ ] Delete Channel Permission
  - [ ] Delete Channel Tag
  - [ ] Delete Read State
  - [ ] Delete Safety Warnings
  - [ ] Follow Channel
  - [ ] Get Active Threads
  - [ ] Get Call Eligibility
  - [ ] Get Channel
  - [ ] Get Channel Post Data
  - [ ] Get DM Channel
  - [ ] Get Guild Active Threads
  - [ ] Get Guild Channels
  - [ ] Get Guild Top Read Channels
  - [ ] Get Joined Private Archived Threads
  - [ ] Get Private Archived Threads
  - [x] Get Private Channels
  - [ ] Get Public Archived Threads
  - [ ] Get Supplemental Message Request Data
  - [ ] Get Thread Member
  - [ ] Get Thread Members
  - [ ] Join Thread
  - [ ] Leave Thread
  - [ ] Modify Call
  - [x] Modify Channel
  - [ ] Modify Channel Permissions
  - [ ] Modify Channel Status
  - [ ] Modify Channel Tag
  - [ ] Modify Guild Channel Positions
  - [ ] Modify Thread Settings
  - [ ] Reject Message Request
  - [x] Remove Channel Recipient
  - [ ] Remove Thread Member
  - [ ] Report Safety Warning False Positive
  - [ ] Ring Channel Recipients
  - [ ] Search Threads
  - [ ] Stop Ringing Channel Recipients
  - [ ] Trigger Typing Indicator
  - [ ] Update Message Request
- **Clan**
  - [ ] Create Clan
  - [ ] Disable Clan
  - [ ] Get Clan
  - [ ] Get Clan Settings
  - [ ] Modify Clan Settings
  - [ ] Set Clan Identity
- **Discovery**
  - [ ] Add Guild Discovery Subcategory
  - [ ] Get Discoverable Guilds
  - [ ] Get Discovery Categories
  - [ ] Get Discovery Slug
  - [ ] Get Guild Discovery Metadata
  - [ ] Get Guild Discovery Requirements
  - [ ] Modify Guild Discovery Metadata
  - [ ] Remove Guild Discovery Subcategory
  - [ ] Search Discoverable Guilds
  - [ ] Search Published Guilds
  - [ ] Validate Discovery Search Term
- **Emoji**
  - [ ] Create Application Emoji
  - [ ] Create Guild Emoji
  - [ ] Delete Application Emoji
  - [ ] Delete Guild Emoji
  - [ ] Get Application Emoji
  - [ ] Get Application Emojis
  - [ ] Get Emoji Guild
  - [ ] Get Emoji Source
  - [ ] Get Guild Emoji
  - [ ] Get Guild Emojis
  - [ ] Get Guild Top Emojis
  - [ ] Modify Application Emoji
  - [ ] Modify Guild Emoji
- **Family center**
  - [ ] Create Linked User Request
  - [ ] Get Family Center Overview
  - [ ] Get Link Code
  - [ ] Get Linked Users
  - [ ] Modify Linked User
- **Guild scheduled events**
  - [ ] Create Guild Scheduled Event
  - [ ] Create Guild Scheduled Event Exception
  - [ ] Create Guild Scheduled Event Exception User
  - [ ] Create Guild Scheduled Event User
  - [ ] Delete Guild Scheduled Event
  - [ ] Delete Guild Scheduled Event Exception
  - [ ] Delete Guild Scheduled Event Exception User
  - [ ] Delete Guild Scheduled Event User
  - [ ] Get Guild Scheduled Event
  - [ ] Get Guild Scheduled Event Exception Users
  - [ ] Get Guild Scheduled Event User Count
  - [ ] Get Guild Scheduled Event Users
  - [ ] Get Guild Scheduled Events
  - [ ] Get User Guild Scheduled Events
  - [ ] Modify Guild Scheduled Event
  - [ ] Modify Guild Scheduled Event Exception
- **Guild template**
  - [ ] Create Guild Template
  - [ ] Delete Guild Template
  - [ ] Get Guild Template
  - [ ] Get Guild Templates
  - [ ] Modify Guild Template
  - [ ] Sync Guild Template
  - [ ] Use Guild Template
- **Guild**
  - [ ] Acknowledge DM Settings Upsell Modal
  - [ ] Action Guild Join Request
  - [ ] Action Guild Join Request by User
  - [ ] Add Guild Member
  - [ ] Add Guild Member Role
  - [ ] Add Guild Role Members
  - [ ] Bulk Action Guild Join Requests
  - [ ] Bulk Guild Ban
  - [x] Create Guild
  - [ ] Create Guild Ban
  - [ ] Create Guild Join Request
  - [ ] Create Guild Join Request Interview
  - [ ] Create Guild Role
  - [x] Delete Guild
  - [ ] Delete Guild Ban
  - [ ] Delete Guild Join Request
  - [ ] Delete Guild Role
  - [ ] Get Admin Community Eligibility
  - [ ] Get Guild
  - [ ] Get Guild Ban
  - [ ] Get Guild Bans
  - [ ] Get Guild Basic
  - [ ] Get Guild Join Request
  - [ ] Get Guild Join Request Cooldown
  - [ ] Get Guild Join Requests
  - [ ] Get Guild Member
  - [ ] Get Guild Member Verification
  - [ ] Get Guild Members
  - [ ] Get Guild Members Supplemental
  - [ ] Get Guild Onboarding
  - [ ] Get Guild Preview
  - [ ] Get Guild Prune
  - [ ] Get Guild Role
  - [ ] Get Guild Role Member Counts
  - [ ] Get Guild Role Members
  - [ ] Get Guild Roles
  - [ ] Get Guild Vanity Invite
  - [ ] Get Guild Welcome Screen
  - [ ] Get Guild Widget
  - [ ] Get Guild Widget Image
  - [ ] Get Guild Widget Settings
  - [ ] Get Join Request Guilds
  - [ ] Get User Guilds
  - [ ] Join Admin Community
  - [ ] Join Guild
  - [ ] Join Wumpus Feedback Squad
  - [ ] Leave Guild
  - [ ] Modify Current Guild Member
  - [ ] Modify Current Guild Member Nick
  - [ ] Modify Guild
  - [ ] Modify Guild Member
  - [ ] Modify Guild Member Profile
  - [ ] Modify Guild Member Verification
  - [ ] Modify Guild MFA Level
  - [ ] Modify Guild Onboarding
  - [ ] Modify Guild Role
  - [ ] Modify Guild Role Positions
  - [ ] Modify Guild Vanity Invite
  - [ ] Modify Guild Welcome Screen
  - [ ] Modify Guild Widget
  - [ ] Prune Guild
  - [ ] Query Guild Members
  - [ ] Remove Guild Member
  - [ ] Remove Guild Member Role
  - [ ] Reset Guild Join Request
  - [ ] Search Guild Bans
  - [ ] Search Guild Members
- **Integration**
  - [ ] Delete Channel Integration
  - [ ] Delete Guild Integration
  - [ ] Enable Guild Integration
  - [ ] Get Channel Integrations
  - [ ] Get Guild Integration Application IDs
  - [ ] Get Guild Integrations
  - [ ] Get Suggested GIF Search Terms
  - [ ] Get Trending GIF Categories
  - [ ] Get Trending GIF Search Terms
  - [ ] Get Trending GIFs
  - [ ] Join Integration Guild
  - [ ] Migrate Guild Command Scope
  - [ ] Modify Guild Integration
  - [ ] Search GIFs
  - [ ] Search Tenor GIFs
  - [ ] Sync Guild Integration
  - [ ] Track Selected GIF
- **Invite**
  - [ ] Accept Invite
  - [x] Create Channel Invite
  - [ ] Create User Invite
  - [x] Delete Invite
  - [x] Get Channel Invites
  - [x] Get Guild Invites
  - [ ] Get Invite
  - [ ] Get User Invites
  - [ ] Revoke User Invites
- **Message**
  - [ ] Acknowledge Message
  - [ ] Acknowledge Pinned Messages
  - [ ] Create Attachments
  - [x] Create DM Message
  - [ ] Create Greet Message
  - [x] Create Message
  - [ ] Create Poll Vote
  - [ ] Create Reaction
  - [ ] Crosspost Message
  - [ ] Delete All Reactions
  - [ ] Delete Attachment
  - [ ] Delete Conversation Summary
  - [x] Delete DM Message
  - [x] Delete Message
  - [ ] Delete Own Reaction
  - [ ] Delete Reaction
  - [ ] Delete Reaction Emoji
  - [x] Edit DM Message
  - [x] Edit Message
  - [ ] End Poll
  - [ ] Get Answer Voters
  - [ ] Get Channel Media Preview
  - [ ] Get Conversation Summaries
  - [x] Get Message
  - [x] Get Messages
  - [x] Get Pinned Messages
  - [ ] Get Reactions
  - [ ] Hide Message from Guild Feed
  - [x] Pin Message
  - [ ] Preload Messages
  - [ ] Refresh Attachment URLs
  - [x] Search Messages
  - [ ] Unfurl Embed
  - [ ] Unfurl Embeds
  - [x] Unpin Message
- **Premium referrals**
  - [ ] Create Premium Referral
  - [ ] Get Premium Referral
  - [ ] Get Premium Referral Eligibility
  - [ ] Get Premium Referral Eligible Users
  - [ ] Preview Premium Referral
- **Presence**
  - [ ] Create Headless Session
  - [ ] Delete Headless Session
  - [ ] Get Activity Metadata
  - [ ] Get Activity Secret
  - [ ] Get Presences
  - [ ] Update Presence
- **Quest**
  - [ ] Accept Quest
  - [ ] Claim Quest Reward
  - [ ] Complete Quest
  - [ ] Dismiss Quest Content
  - [ ] Get Available Quests
  - [ ] Get Quest Reward Code
  - [ ] Reset Quest
  - [ ] Reset Quest Dismissibility
  - [ ] Send Quest Heartbeat
- **Relationship**
  - [ ] Bulk Remove Relationships
  - [ ] Create Game Relationship
  - [ ] Create Game Relationship by Application
  - [ ] Create Relationship
  - [ ] Get Friend Suggestions
  - [ ] Get Game Relationships
  - [ ] Get Relationships
  - [ ] Ignore User
  - [ ] Modify Relationship
  - [ ] Remove Friend Suggestion
  - [ ] Remove Game Relationship
  - [ ] Remove Game Relationship by Application
  - [ ] Remove Relationship
  - [ ] Send Friend Request
  - [ ] Send Game Friend Request
  - [ ] Unignore User
- **Soundboard**
  - [ ] Create Guild Soundboard Sound
  - [ ] Delete Guild Soundboard Sound
  - [ ] Get Default Soundboard Sounds
  - [ ] Get Guild Soundboard Sound
  - [ ] Get Guild Soundboard Sounds
  - [ ] Get Soundboard Sound Guild
  - [ ] Modify Guild Soundboard Sound
  - [ ] Send Soundboard Sound
- **Stage instance**
  - [ ] Delete Stage Instance
  - [ ] Get Stage Instance
  - [ ] Modify Stage Instance
- **Sticker**
  - [ ] Create Guild Sticker
  - [ ] Delete Guild Sticker
  - [ ] Get Guild Sticker
  - [ ] Get Guild Stickers
  - [ ] Get Sticker
  - [ ] Get Sticker Guild
  - [ ] Get Sticker Pack
  - [ ] Get Sticker Packs
  - [ ] Modify Guild Sticker
- **Team**
  - [ ] Accept Team Invite
  - [ ] Add Team Member
  - [ ] Create Company
  - [ ] Create Team
  - [ ] Delete Team
  - [ ] Get Team
  - [ ] Get Team Applications
  - [ ] Get Team Members
  - [ ] Get Team Payout Onboarding
  - [ ] Get Team Payout Report
  - [ ] Get Team Payouts
  - [ ] Get Team Stripe Connect URL
  - [ ] Get Teams
  - [ ] Modify Team
  - [ ] Modify Team Member
  - [ ] Remove Team Member
  - [ ] Search Companies
- **User settings**
  - [x] Change username
  - [x] Change global name
  - [ ] Request mail change
  - [x] Change email
  - [x] Change password
  - [x] Change avatar
  - [x] Change banner
  - [x] Change bio
  - [x] Change pronouns
  - [x] Change accent color
  - [x] Change date of birth
  - [ ] Change flags
  - [ ] Change avatar decoration
  - [ ] Bulk Modify User Guild Settings
  - [ ] Create Notification Settings Snapshot
  - [ ] Delete Notification Settings Snapshot
  - [ ] Get Email Settings
  - [ ] Get Notification Settings Snapshots
  - [ ] Get User Consents
  - [ ] Get User Settings
  - [ ] Get User Settings Proto
  - [ ] Modify Notification Settings
  - [ ] Modify User Consents
  - [ ] Modify User Guild Settings
  - [ ] Modify User Settings
  - [ ] Modify User Settings Proto
  - [ ] Restore Notification Settings Snapshot
- **User**
  - [ ] Apply Confetti Potion
  - [ ] Authorize User Connection
  - [ ] Confirm Tutorial Indicator
  - [ ] Create Contact Sync Connection
  - [ ] Create Domain Connection
  - [ ] Create Pomelo Migration
  - [ ] Create User Connection Callback
  - [ ] Create User Harvest
  - [ ] Create WebAuthn Authenticator
  - [ ] Delete Recent Mention
  - [ ] Delete User
  - [ ] Delete User Connection
  - [ ] Delete WebAuthn Authenticator
  - [ ] Disable SMS MFA
  - [ ] Disable TOTP MFA
  - [ ] Disable User
  - [ ] Enable SMS MFA
  - [ ] Enable TOTP MFA
  - [ ] Get Backup Codes
  - [ ] Get Channel Affinities
  - [ ] Get Confetti Potions
  - [ ] Get Current User
  - [ ] Get Friend Token
  - [ ] Get Guild Affinities
  - [ ] Get Mutual Relationships
  - [ ] Get Pomelo Eligibility
  - [ ] Get Pomelo Suggestions
  - [ ] Get Recent Mentions
  - [ ] Get User
  - [ ] Get User Affinities
  - [ ] Get User Affinities v2
  - [ ] Get User Connection Access Token
  - [ ] Get User Connection Subreddits
  - [ ] Get User Connections
  - [ ] Get User Harvest
  - [ ] Get User Note
  - [ ] Get User Notes
  - [ ] Get User Premium Usage
  - [ ] Get User Profile
  - [ ] Get User Profile Effects
  - [ ] Get WebAuthn Authenticators
  - [ ] Join Active Developer Program
  - [ ] Join HypeSquad Online
  - [ ] Leave Active Developer Program
  - [ ] Leave HypeSquad Online
  - [ ] Modify Current User
  - [ ] Modify Current User Account
  - [ ] Modify User Connection
  - [ ] Modify User Email
  - [ ] Modify User Note
  - [ ] Modify User Profile
  - [ ] Modify WebAuthn Authenticator
  - [ ] Refresh User Connection
  - [ ] Send Backup Codes Challenge
  - [ ] Suppress Tutorial
  - [ ] Verify User Captcha
  - [ ] Verify User Email Change
- **Voice**
  - [ ] Broadcast Stream Notification
  - [ ] Get Current User Voice State
  - [ ] Get Guild Voice Regions
  - [ ] Get Stream Preview
  - [ ] Get User Voice State
  - [ ] Get Voice Filters Catalog
  - [ ] Get Voice Regions
  - [ ] Modify Current User Voice State
  - [ ] Modify User Voice State
  - [ ] Send Voice Channel Effect
  - [ ] Upload Stream Preview
- **Webhook**
  - [ ] Create Webhook
  - [ ] Delete Webhook
  - [ ] Delete Webhook Message
  - [ ] Delete Webhook with Token
  - [ ] Edit Webhook Message
  - [ ] Execute GitHub-Compatible Webhook
  - [ ] Execute Slack-Compatible Webhook
  - [ ] Execute Webhook
  - [ ] Get Channel Webhooks
  - [ ] Get Guild Webhooks
  - [ ] Get Webhook
  - [ ] Get Webhook Message
  - [ ] Get Webhook with Token
  - [ ] Modify Webhook
  - [ ] Modify Webhook with Token
