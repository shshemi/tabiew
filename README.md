# Tabiew

Tabiew is a lightweight, terminal-based application to view and query delimiter separated value formatted documents, such as CSV or TSV files.

![Image Alt text](/images/screenshot.png "Screenshot")

## Features

- 📊 **Table View**: Explore your data with an easy-to-navigate table view.
- 🔍 **Sheet View**: Dive deep into records with details of individual records.
- 🛠️ **SQL Query Capability**: Utilize SQL for powerful data querying, such as filtering, sorting, and aggregations.
- 🗂️ **Multiple Table Support**: Work with multiple tables simultaneously.
- ⌨️ **Vim Inspired Keybindings**: Navigate through your data effortlessly using Vim-style keybindings.

## Keybindings️

|Key Combination|Functionality|
|-|-|
| `v`| Switch view|
| `k` or `Arrow Up`| Move up in the table or scroll up in sheet view|
| `j` or `Arrow Down`| Move down in the table or scroll down in sheet view|
| `h` or `Arrow Left`| Move to the previous item in sheet view|
| `l` or `Arrow Right`| Move to the next item in sheet view|
| `Page Up` or  `Ctrl+b`| Move one page up|
| `Page Down` or `Ctrl+f`| Move one page down|
| `H`| Select previous tab|
| `L`| Select next tab|
| `Ctrl+u`| Move up half a page|
| `Ctrl+d`| Move down half a page|
| `Home` or `g`| Move to the first row|
| `End` or `G`| Move to the last row|
| `R`| Select a random row|
| `q`| Close current tab|
| `:`| Command mode|

## Commands
|Command|Example|Description|
|-|-|-|
|`:Q` or `:query`|`:Q SELECT * FROM df`|Query the data in Structured Query Language(SQL). The table name is the file name without extension|
|`:S` or `:select`| `:S price, area, bedrooms, parking`|Query current data frame for columns/functions|
|`:F` or `:filter`| `:F price < 20000 AND bedrooms > 4`|Filter current data frame, keeping rows were the condition(s) match|
|`:O` or `:order`| `:O area`|Sort current data frame by column(s)|
|`:tabn`| `:tabn SELECT * FORM user WHERE balance > 1000`|Sort current data frame by column(s)|
|`:q` or `:quit` |`:q`| Return to table from sheet view otherwise quit|
|`:schema`| `:schema`| Show loaded data frame(s) alongside their path(s)|
|`:reset`| `:reset`| Reset the table to the original data frame|
|`:help`| `:help`| Show help menu|

## Tutorial

For a guide on using Tabiew, including instructions on opening files, navigating tables, performing queries, and using inline queries, please read to the [tutorial](https://github.com/shshemi/tabiew/blob/main/tutorial/tutorial.md).

## Installation

There are various ways to install Tabiew:

### Arch Linux
You can install from the [official repositories](https://archlinux.org/packages/extra/x86_64/tabiew/) using [pacman](https://wiki.archlinux.org/title/pacman):
```bash
pacman -S tabiew
```

### Debian-based
Download the `.deb` package from the [GitHub releases page](https://github.com/shshemi/tabiew/releases) and run:
 ```bash
 sudo dpkg -i <path_to_package.deb>
 ```

### RPM-based
Download the `.rpm` package from the [GitHub releases page](https://github.com/shshemi/tabiew/releases) and run:
 ```bash
 sudo rpm -i <path_to_package.rpm>
 ```

### MacOS
Installing Tabiew using [Homebrew](https://brew.sh/):
 ```bash
 brew install shshemi/tabiew/tabiew
 ```

### Windows
Download the `.exe` file from the [GitHub releases page](https://github.com/shshemi/tabiew/releases) and place it in a directory that is included in the system's PATH environment variable.

**Disclaimer:** The Windows version may experience performance lag.

### Cargo
Installing Tabiew from *Crates.io*:
 ```bash
 cargo install tabiew
 ```

### Build from Source
Ensure you have the rustc version 1.80 (or higher) installed, then:
```bash
git clone https://github.com/shshemi/tabiew.git
cd tabiew
cargo build --release
cp ./target/release/tw <system_or_local_bin_path>
```

## Usage
Start Tabiew with `tw`
```bash
tw <path_to_csv(s)>
```
Options:
- `--no-header`: Use this option if the CSV file does not contain a header row.
- `--ignore-errors`: Ignore parsing errors while loading the CSV file.
- `--infer-schema`: Set the schema inference method. Options are no, fast, full, and safe.
- `--quote-char`: Set the quote character.
- `--separator`: Set the separator character.
- `--theme`: Set the theme.

To open a TSV file use:
```bash
tw <path_to_tsv(s)> --separator $'\t' --no-header
```
## Themes
### Monokai (default):
![Image Alt text](/images/theme-monokai.png "Monokai")

### Argonaut:
![Image Alt text](/images/theme-argonaut.png "Argonaut")

### Terminal:
![Image Alt text](/images/theme-terminal.png "Terminal")

## Contributing
Contributions are welcome! Please fork the repository and submit pull requests with your features and bug fixes.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
