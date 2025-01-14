# Harakiri-Rust
This the first **Discord Raid Bot** made in RustLang
I recommend you use with a VPN or a Proxy to evade Discord Ratelimit.
If bot doesn't start change your IP Using a VPN / Proxy.

## Requirements
This is Rust Lang, you should install with the [Official Rust Installer](https://rustup.rs/)

### Linux
In Linux you can use this command to install rust:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Compiling
To compile Binary File for Portable Using:
```sh
cargo build --release
```

To compile and run:
```sh
cargo run --release
```

## Configuration
The configuration object is on lines `[45-56]`.

#### Fields
Here are a example configuration:
```rs
const CONFIG: Config = Config {
    token: "yourbottoken",
    prefix: "$",
    presence: "github.com/mcraxker",
    channel_name: "raided-by-harakiri",
    channel_message: "@everyone https://github.com/mcraxker hahahha",
    webhook_name: "Killed by Harakiri",
    dm_message: "Raided by harakiri bot lol",
    guild_name: "Harikiri on top",
    guild_icon: Some(""),
    role_name: "github.com/mcraxker lol",
};
```

## Responsability
Im not responsable of the bad use that can you have using that bot.

## Support
If you need Support with bot, dm me on telegram, https://t.me/mcraxker