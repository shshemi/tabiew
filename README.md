# Tabiew

Tabiew is a lightweight, terminal-based application designed to help view and query CSV files directly in your terminal. It utilizes Polar data frames and its querying engine to perform complex data manipulations without the need for a database system.

![Image Alt text](/images/screenshot.png "Screenshot")

## Features

- 📊 **Table View**: Explore your data with an easy-to-navigate table view.
- 🔍 **Detailed View**: Dive deeper with a detailed view of individual records to inspect specific data entries.
- 🛠️ **SQL Query Capability**: Utilize SQL for powerful data querying within the app to perform filtering, sorting, and aggregations directly from your terminal.
- ⌨️ **Vim Inspired Keybindings**: Navigate through your data effortlessly using Vim-style keybindings.

## Keybindings️

|Key Combination|Functionality|
|-|-|
| `v`| Toggle detailed view|
| `k` or `Arrow Up`| Move up in the table or scroll up in detailed view|
| `j` or `Arrow Down`| Move down in the table or scroll down in detailed view|
| `h` or `Arrow Left`| Move to the previous item in detailed view|
| `l` or `Arrow Right`| Move to the next item in detailed view|
| `Page Up` or  `Ctrl+b`| Move one page up|
| `Page Down` or `Ctrl+f`| Move one page down|
| `Ctrl+u`| Move up half a page|
| `Ctrl+d`| Move down half a page|
| `Home` or `g`| Move to the first row|
| `End` or `G`| Move to the last row|
| `q`| Quit|
| `:`| Command mode|

## Commands
|Command|Usage|Description|
|-|-|-|
|`:Q` or `:query`|`:Q <query>`| Query the data in Structured Query Language (SQL). Table name is 'df'|
|`:q` or `:quit` |`:q`| Quit Tabiew|
|`:goto`| `:goto <line_index>`| Jumps to the specified line index|
|`:moveup`| `:moveup <lines>`| Jump a specified number of lines up|
|`:movedown`| `:movedown <lines>`| Jump a specified number of lines down|
|`:reset`| `:reset`| Reset the original data frame|
|`:help`| `:help`| Show help menu|
|`:S` or `:select`| `:select <column_name(s)>`|Query the original for selected columns|
|`:F` or `:filter`| `:filter <condition(s)>`|Query the original dataset where the condition(s) match|
|`:O` or `:order`| `:order <column(s)_and_order(s)>`|Query the original data frame ordering by requested columns|

## Installation

You can install Tabiew via three methods:

1. **Cargo**: Install Tabiew from *Crates.io* is the simplest method if there is no intention to modify the source code.
    ```bash
    cargo install tabiew
    ```

1. **deb Package**: Download the .deb package directly from our [GitHub releases page](https://github.com/shshemi/tabiew/releases) for Debian-based Linux distributions.

    ```bash
    sudo dpkg -i <path_to_package.deb>
    ```

1. **Build from Source**: If you prefer, you can also build the application from source. Clone the repository and follow the build instructions provided in the README file.

    Make sure you have the 1.80.0-nightly (or higher) version of Rust installed.

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
