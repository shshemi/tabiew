# Tabiew

Tabiew is a lightweight, terminal-based application designed to help view and query CSV files directly in your terminal. It utilizes Polar data frames and its querying engine to perform complex data manipulations without the need for a database system.

![Image Alt text](/images/screenshot.png "Screenshot")

## Features 🌟

- 📊 **Table View**: Explore your data with an easy-to-navigate table view.
- 🔍 **Detailed View**: Dive deeper with a detailed view of individual records to inspect specific data entries.
- 🛠️ **SQL Query Capability**: Utilize SQL for powerful data querying within the app to perform filtering, sorting, and aggregations directly from your terminal.
- ⌨️ **Vim Inspired Keybindings**: Navigate through your data effortlessly using Vim-style keybindings.

## Installation

You can install Tabiew via two methods:

 1. **deb Package**: Download the .deb package directly from our [GitHub releases page](https://github.com/shshemi/tabiew/releases). This method is suitable for Debian-based Linux distributions.

    ```bash
    sudo dpkg -i <path_to_package.deb>
    ```

 1. **Build from Source**: If you prefer, you can also build the application from source. Clone the repository and follow the build instructions provided in the README file.

    Make sure you have the 1.80.0-nightly version of Rust installed.

    ```bash
    git pull https://github.com/shshemi/tabiew.git
    cd tabiew
    rustup override set nightly
    cargo build --release
    cp ./target/release/tabiew <system_or_local_bin_path>
    ```

## Contributing
Contributions are welcome! Please fork the repository and submit pull requests with your features and bug fixes.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
