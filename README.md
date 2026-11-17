# Important

> [!WARNING]
> I'm pausing this project until they release the new Rust discord. The main reason is that [wreq](https://github.com/0x676e67/wreq) still not support H/3 and the "best" solution would be to fork the chromium project to make a library that exposes Cronet to minic 1:1 chrome H/2 and H/3 behaviour but it will be a real pain and anyone who wants to compile the project will have to clone and compile Chromium (which takes several minutes and is about 100 GB).
> 
> So the official Rust client will have to use in H/2 or H/3 with [quiche](https://github.com/cloudflare/quiche), but it will be very easy to emulate.

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
- **Automatic heartbeat** with sequence number handling
- **Captcha handling**
- **Queue system and rate limiting**

## Crates

- **[discord_client_gateway](./discord_client_gateway)**: A high-level Rust implementation of the Discord gateway, designed to provide a connection to Discord's real-time WebSocket API. See the [README](./discord_client_gateway/README.md) for more information.
- **[discord_client_rest](./discord_client_rest)**: A high-level Rust implementation of the Discord REST API, designed to provide a connection to Discord's HTTP API. See the [README](./discord_client_rest/README.md) for more information.
- **[discord_client_structs](./discord_client_structs)**: A collection of structs and enums that represent the data structures used by Discord's API.
