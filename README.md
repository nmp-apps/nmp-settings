# nmp-settings

**One unified, user-friendly interface to configure all your applications.**

<img width="115.4736" height="162.71280000000002" alt="logo" src="https://github.com/user-attachments/assets/907e5bea-bf6e-4467-9187-55de842c11bc" align="right" />

---

🛠️ Stop chasing configuration files.  
👨‍💻 Simplify your workflow as a developer.  
🎛️ Put every setting in one beautiful place.

---

## ✨ What is nmp-settings?

**nmp-settings** is a modular, plugin-based configuration manager that brings **all your application settings into a single interface**—whether you're a casual user or a power developer.

No more hunting for obscure config files, diving into distribution-specific settings, or explaining setup instructions over and over.  
**nmp-settings** offers a unified, visual way to manage settings across apps and systems—powered by simple, cross-language plugins.

---

## 🎯 Key Features

- 🧩 **Plugin-Based Architecture**  
  Easily extend nmp-settings with custom plugins for any application.

- 🗂️ **Smart Categorization**  
  Settings are grouped into intuitive categories (e.g., Display, Sound, Network).

- 🧠 **Remember Everything**  
  Each setting is tracked by its plugin, and auto-saved to reduce redundancy.

- 💬 **User Prompts**  
  Ask confirmation questions before applying sensitive changes.

- 🔄 **Live Application of Settings**  
  Plugins receive updated config data as JSON and apply changes instantly.

---

## 🧠 Concept Overview

`nmp-settings` is not another config editor—it's a **visual orchestrator** of settings powered by plugins.

Each plugin:

- Declares its settings in a simple **JSON schema**
- Receives updated settings via `--json` CLI flag (e.g. `nmp-settings-plugin-hyprpaper --json '{...}'`)
- Is responsible only for applying settings, not building UI

### How It Works

1. Plugins provide a config JSON with fields and metadata.
2. `nmp-settings` renders the UI automatically based on that JSON.
3. User updates the settings.
4. On save, the updated JSON is passed back to the plugin via CLI.
5. If a field includes a **question**, a confirmation dialog appears before applying.

---

## 🧑‍💻 For Plugin Developers

nmp-settings is built to empower developers with:

1. ✅ A **unified API** across all plugins
2. 🖼️ **Zero UI code** required – just focus on logic
3. 💬 **Cross-language compatibility** (use any language you prefer)
4. 🧪 A simple and predictable **plugin structure**
5. 🤝 A community-driven approach to plugin standards

**Start developing a plugin today!** Check out our [Developer Wiki](https://github.com/nmp-apps/nmp-settings/wiki) for step-by-step instructions.

---

## 🧑‍🎨 For End Users

Using nmp-settings, you get:

1. 🖥️ **All your settings in one place**
2. 🔍 **Easy-to-navigate categories**
3. 🎉 **One-click plugin discovery and installation** [NOT IMPLEMENTED]
4. 🌈 **A clean and modern UI experience**

---

## 📦 Example Plugins

- [nmp-settings-plugin-example](https://github.com/nmp-apps/nmp-settings-plugin-example)
- *(more coming soon!)*

> Want your favorite tool supported? [Request a plugin](https://github.com/nmp-apps/nmp-settings/issues/new) or [create one yourself](https://github.com/nmp-apps/nmp-settings/wiki/Plugins).

---

## 📚 Documentation

- **Getting Started**: [Installation Guide](https://github.com/nmp-apps/nmp-settings/wiki/Installation)
- **How Plugins Work**: [Plugin System Overview](https://github.com/nmp-apps/nmp-settings/wiki/Plugins)

---

## 🚀 Installation

> 📦 Coming soon to [AppImageHub / AUR / Homebrew / etc.]

For now, download binaries or clone and build from source:
1. Install rust
2. Clone and bundle
```bash
git clone https://github.com/nmp-apps/nmp-settings.git
cd nmp-settings
sh bundle-[appimage/deb/default].sh
```
3. After that you will see path with your bundled app

More detailed instructions, [here](https://github.com/nmp-apps/nmp-settings/wiki/Installation)

## 🙌 Contributing

We welcome contributions of all kinds—bug reports, plugins, documentation, translations, and more.

## 🧑‍🔧 Maintainers & Community

Maintainer: keiko37

Contributors: [See all](https://github.com/nmp-apps/nmp-settings/graphs/contributors)

## 📄 License

[GNU GPLv3](https://github.com/nmp-apps/nmp-settings/tree/main?tab=GPL-3.0-1-ov-file)

## 🌟 Support the Project

If you like the project, consider giving us a ⭐ on GitHub and sharing it with others!
