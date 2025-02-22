# Discord client RS

This repository contains a collection of crates that provide tools for building Discord clients in Rust.

It tends to provide a high-level API for interacting with Discord's real-time WebSocket API and REST API, as well as a collection of structs and enums that represent the data structures used by Discord's API.

## Key Features

- **High-level API**
- **Automatic reconnection**
- **Event dispatching**
- **Guilds subscribing**
- **Zlib-stream decompression** support for Discord's compressed payloads
- **TLS impersonation** and **HTTP/2 (H2) mimicing** of Chrome to avoid detection
- **Undetectable** by Discord's anti-bot mechanisms unless you do something stupid or spammy

## Crates

- **[discord_client_gateway](./discord_client_gateway)**: A high-level Rust implementation of the Discord gateway, designed to provide a connection to Discord's real-time WebSocket API. See the [README](./discord_client_gateway/README.md) for more information.
- **[discord_client_rest](./discord_client_rest)**: A high-level Rust implementation of the Discord REST API, designed to provide a connection to Discord's HTTP API. See the [README](./discord_client_rest/README.md) for more information.
- **[discord_client_structs](./discord_client_structs)**: A collection of structs and enums that represent the data structures used by Discord's API.