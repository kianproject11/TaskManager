# 🧠 TaskManager

Your PC feels slow. Let's find out why, in plain English.

Most task managers look like airplane cockpits: 50 columns, cryptic names like svchost.exe, and zero explanation. TaskManager is different. It's small, calm and friendly, and it tells you what's going on in words a human can understand.

> ⚠️ Early days! Right now it's a tiny terminal app. The pretty version is coming.

## ✨ What we're building

- 🗣️ Human names. svchost.exe becomes "Windows Services"
- 📦 Grouped apps. "Chrome: 12 processes, 2.1 GB", not 12 confusing rows
- 🐢 "Why is my PC slow?" button. One click, one clear answer
- 🛡️ Safe closing. A warning before you close anything important
- 🪶 Featherweight. A task manager shouldn't hog your RAM, so ours is written in Rust to stay tiny
- 🎨 Clean and modern look. Easy on the eyes, not a wall of numbers

## 🚦 Status

| Version | What it does | State |
|---|---|---|
| v0.0.1 | Lists the top processes in the terminal | ✅ Done |
| v0.1 | Window, live CPU/RAM, grouped list, safe close | 🔨 Next |
| v0.2 | Friendly names, startup manager, search | 📝 Planned |
| v0.3 | "Why is my PC slow?", usage history | 📝 Planned |

Windows first, Linux later.

## 🚀 Try it

You need [Rust](https://rustup.rs) installed.

    git clone https://github.com/kianproject11/TaskManager.git
    cd TaskManager
    cargo run

## 🤝 Want to help?

You don't need to know Rust! Soon, friendly process names will live in a simple text file. If you know what a process does, you can add it in one line. Ideas, bug reports and feedback are welcome in [Issues](https://github.com/kianproject11/TaskManager/issues).

## 📄 License

MIT