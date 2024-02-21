<img src="./assets/ghost.png" height=140>&nbsp;&nbsp;<img src="./assets/discord.svg" height=140>

# Ghost Webhook Proxy

I don't like how Ghost's implementation of Slack webhooks looks when used with Discord. So, I decided to make a lightweight proxy in Rust to make them look better.

- Uses ~10MB of RAM on average and has negligible CPU overhead
- Perfect for running alongside Ghost on a budget cloud instance

![New Look](./assets/ghwh_new_embed_look.png)

I'll dockerize it eventually.

## Usage

```bash
$ ghwh-proxy [port]
```

## Installing

1. Download the tool from [releases](https://github.com/Azure-Agst/ghwh-proxy/releases) and onto your server.
2. Install it somewhere useful and configure it to start automatically
    - `cp ghwh-proxy /usr/local/bin/`
    - I've included a systemd defintion you can use [here](./ghwh-proxy.service). Just put in it in `/lib/systemd/system/` and ensure it has permissions `0644`.
    - Then you can use `systemd enable` and `systemd start` to start the service.
3. Generate a discord webhook that you'd like to post your messages to
    - e.g. `https://discord.com/api/webhooks/1234567890/eyJtZXNzYWdlIjoiaGkifQ==`
4. Make a new custom integration in Ghost that posts to your service when the "Post published" trigger occurs
    - The URL you will need to use is the Discord webhook, with `https://discord.com/api/webhooks/` replaced with a link to the discord subdirectory of your proxy service.
    - i.e. `http://127.0.0.1:7120/discord/1234567890/eyJtZXNzYWdlIjoiaGkifQ==`
5. Profit?

## License

This file is distributed under the GNU GPLv3 license. I offer no promises that this project will be maintained into the future. 👍

A full copy of the license can be found in [LICENSE](./LICENSE).

---

*Copyright (C) 2024 Andrew "Azure-Agst" Augustine*