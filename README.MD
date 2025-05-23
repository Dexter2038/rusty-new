# 🦀 rise (rust initial setup engine)

> The unofficial, community-driven way to kickstart modern Rust projects.

Whether you're building a web API, CLI tool, game, or something embedded and weird, 
`rise` helps you spin up opinionated templates fast — with support for async runtimes, 
frameworks like Axum, Bevy, and more.

Choose your stack, generate the boilerplate, and ship something spicy.

---

## ⚠️ Not published on `crates.io` yet

This project is still under active development. There's no `cargo install rise` — **yet**.

To try it out, you'll need to clone this repo and build it locally:

```bash
git clone https://github.com/Dexter2038/rise
cd rise
cargo build --release
```

You can then run it from the target directory:

```bash
./target/release/rise
```

💡 Powered by the Rustacean community
The goal is to support multiple kinds of Rust projects with plug-and-play templates:

Web apps (e.g. Axum, Leptos, Yew, etc.)

CLI tools

Games (Bevy, macroquad, etc.)

Embedded apps

Libraries

And more

Bring your own templates or use the ones bundled in. The vision is modular, async-friendly, and as little boilerplate pain as possible.

🧪 Status: early days
Things are moving fast. Expect rough edges, TODOs, and probably a few panics. Contributions, feedback, and chaos-testing welcome.

It's not official. It's better.


---

Let me know if you want badges, install sections for the future, or a snarkier "FAQ" for people who inevitably ask why it's not on crates.io yet.

## 📝 License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](./LICENSE) file for details.

