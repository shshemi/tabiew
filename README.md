# Tabiew

Tabiew is a lightweight, terminal-based application to view and query delimiter separated value formatted documents, such as CSV and TSV files.

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
|Command|Example|Description|
|-|-|-|
|`:Q` or `:query`|`:Q SELECT * FROM df`| Query the data in Structured Query Language (SQL). The table's name is 'df'|
|`:q` or `:quit` |`:q`| Quit Tabiew|
|`:goto`| `:goto 10`| Jumps to the specified line index|
|`:moveup`| `:moveup 10`| Jump a specified number of lines up|
|`:movedown`| `:movedown 10`| Jump a specified number of lines down|
|`:S` or `:select`| `:S price, area, bedrooms, parking`|Query the original for selected columns|
|`:F` or `:filter`| `:F price < 20000 AND bedrooms > 4`|Query the original dataset where the condition(s) match|
|`:O` or `:order`| `:O area`|Query the original data frame ordering by requested columns|
|`:reset`| `:reset`| Reset the original data frame|
|`:help`| `:help`| Show help menu|

## Installation

There are various ways to install Tabiew:

### Debian-based
Download the `.deb` package from the [GitHub releases page](https://github.com/shshemi/tabiew/releases) then:
 ```bash
 sudo dpkg -i <path_to_package.deb>
 ```

### RPM-based
Download the `.rpm` package from the [GitHub releases page](https://github.com/shshemi/tabiew/releases) then:
 ```bash
 sudo rpm -i <path_to_package.rpm>
 ```

### Windows
Download the `.exe` file from the [GitHub releases page](https://github.com/shshemi/tabiew/releases) and place it in a directory that is included in system's PATH environment variable.

**Disclaimer:** The Windows version may experience performance lag.

### Cargo
Install Tabiew from *Crates.io*:
 ```bash
 cargo install tabiew
 ```

### Build from Source
Ensure you have the 1.80.0-nightly (or higher) version of Rust installed, then:
```bash
git clone https://github.com/shshemi/tabiew.git
cd tabiew
rustup override set nightly
cargo build --release
cp ./target/release/tabiew <system_or_local_bin_path>
```

## Usage
Start Tabiew with `tw`
```bash
tw <path_to_csv>
```
Options:
- `--no-header`: Use this option if the CSV file does not contain a header row.
- `--ignore-errors`: Ignore parsing errors while loading the CSV file.
- `--infer-schema`: Set the schema inference method. Options are no, fast, full, and safe.
- `--quote-char`: Set the quote character.
- `--separator`: Set the separator character.
  
## Contributing
Contributions are welcome! Please fork the repository and submit pull requests with your features and bug fixes.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
