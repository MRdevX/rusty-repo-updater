# Rusty Repo Updater

Automatically update multiple Git repositories with a single command. A lightweight, efficient Rust utility for managing project repositories.

## 🚀 Features

- **Automatic Repository Discovery**: Scans a specified directory for Git repositories
- **Multi-Branch Support**: Fetches and pulls updates for develop, main, and master branches
- **Simple CLI Interface**: Easy-to-use command-line tool
- **Comprehensive Logging**: Provides detailed output about repository update processes

## 📦 Prerequisites

- Rust (latest stable version recommended)
- Git installed and configured on your system

## 🛠️ Installation

### Building from Source

1. Clone the repository:

   ```bash
   git clone https://github.com/MRdevX/rusty-repo-updater.git
   cd rusty-repo-updater
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. The executable will be located in `target/release/rusty-repo-updater`

### Alternative Installation Methods

- Cargo install (coming soon):
  ```bash
  cargo install rusty-repo-updater
  ```

## 💡 Usage

Run the tool and enter the directory path when prompted:

```bash
./rusty-repo-updater
Enter the directory path: /path/to/your/projects
```

### Example Workflow

1. Navigate to the directory containing your project repositories
2. Run the tool
3. Watch as it automatically updates all Git repositories

## 🔧 Customization

Currently supports updating these branches:

- develop
- main
- master

## 🤝 Contributing

Contributions are welcome! Please feel free to:

- Open issues for bugs or feature requests
- Submit pull requests
- Provide feedback

### Development Setup

1. Clone the repository
2. Install Rust via [rustup](https://rustup.rs/)
3. Run tests:
   ```bash
   cargo test
   ```
4. Build the project:
   ```bash
   cargo build
   ```

## 🛡️ Limitations

- Requires write access to Git repositories
- Does not handle repositories with complex authentication
- Assumes standard fetch and pull operations

## 📋 Roadmap

- [ ] Support for custom branch names
- [ ] Parallel repository updates
- [ ] Configuration file support
- [ ] Enhanced error handling

## 📄 License

[MIT License](LICENSE)

## 🌟 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=MRdevX/rusty-repo-updater&type=Date)](https://star-history.com/#MRdevX/rusty-repo-updater)

## 💌 Support

If you find this tool helpful, consider:

- ⭐ Starring the repository
- 📣 Sharing with your network
- 💝 Contributing to the project
