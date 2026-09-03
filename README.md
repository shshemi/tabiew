# Tabiew

Tabiew is a lightweight TUI application that allows users to view and query tabular data files, such as CSV, Parquet, Arrow, and ...

![Image Alt text](https://raw.githubusercontent.com/wiki/shshemi/tabiew/gifs/demo.gif "Screenshot")

## Features

- ⌨️ Vim-style keybindings
- 🛠️ SQL support
- 📊 Support for CSV, TSV, Parquet, JSON, JSONL, Arrow, FWF, Sqlite, Excel, Logfmt, HTML, and Markdown
- 🔍 Fuzzy search
- 🗂️ Multi-table functionality
- 📈 Plotting
- 🎨 More than 400 beautiful themes

## Wiki

Tabiew started as a humble hobby TUI project for viewing CSV data but has evolved to incorporate various features and improvements from valuable community feedbacks. As the project expanded, so did the need for comprehensive documentation, leading to the creation of the [wiki page](https://github.com/shshemi/tabiew/wiki). The wiki offers explanations of features and the best practices to get the most out of Tabiew.

## Installation

There are various ways to install Tabiew:

### Shell (Linux / macOS)

```bash
curl -sS https://raw.githubusercontent.com/shshemi/tabiew/main/install.sh | sh
```

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

Installing Tabiew using [Homebrew](https://brew.sh/) from Homebrew core:
```bash
brew update
brew install tabiew
```
or tap:
 ```bash
brew install shshemi/tabiew/tabiew
 ```

Note: Please be aware that installing Tabiew from the tap involves compiling it from the source, which may take some time to complete.

### Cargo

Installing Tabiew from *Crates.io*:
 ```bash
cargo install --locked tabiew
 ```

### Build from Source

Ensure you have rustc version 1.95 (or higher) installed. Download the desired source version from the [release page](https://github.com/shshemi/tabiew/releases). Extract the downloaded file and navigate into the extracted directory. Then run the following command:
```bash
cargo build --release
cp ./target/release/tw <system_or_local_bin_path>
```

## Usage

Start Tabiew with `tw`
```bash
tw <path_to_file(s)>
```

Tabiew automatically detects the file format based on the file extension. Supported formats include:
- **CSV** (`.csv`) - Comma-separated values
- **TSV** (`.tsv`) - Tab-separated values
- **Parquet** (`.parquet`, `.pqt`)
- **JSON** (`.json`)
- **JSONL** (`.jsonl`) - JSON Lines
- **Arrow** (`.arrow`)
- **FWF** (`.fwf`) - Fixed-width format
- **SQLite** (`.db`, `.sqlite`)
- **Excel** (`.xls`, `.xlsx`, `.xlsm`, `.xlsb`)
- **Avro** (`.avro`)
- **HTML** (`.html`, `.htm`) - reads `<table>` elements
- **Markdown** (`.md`, `.markdown`) - reads pipe tables

Logfmt has no extension of its own, so it needs an explicit format: `tw app.log -f logfmt`.

Examples:

Open various files (format automatically detected):
```bash
tw data.csv data.tsv data.arrow
```

Open CSV files with custom delimiter (pipe-separated):
```bash
tw data.csv --separator '|'
```

Open CSV files with custom delimiter and no header row (semicolon-separated):
```bash
tw data.csv --separator ';' --no-header
```

Override format detection:
```bash
tw data.txt -f parquet
```

Delimiter-separated formats (CSV, TSV, etc):
```bash
# Explicitly use CSV format (comma by default, but can use custom delimiter)
tw data.txt -f csv
tw data.txt -f csv --separator '|'

# Explicitly use TSV format (always tab-delimited)
tw data.txt -f tsv

# Use DSV with custom delimiter (equivalent to csv with --separator)
tw data.txt -f dsv --separator '|'
```

Preview only the first rows of a large file:
```bash
tw big.csv --max-rows 1000
```
`--max-rows` is handy for sanity-checking the schema and a sample of the data before loading a multi-GB file. It applies to CSV/DSV/TSV, Parquet, JSON, JSON Lines, Arrow, and Avro.

Open a URL using curl:
```bash
curl -s "https://raw.githubusercontent.com/wiki/shshemi/tabiew/housing.csv" | tw
```

## Useful Keybindings️

|Key Combination|Functionality|
|-|-|
| `Enter`| Open sheet|
| `f`| In sheet, switch between the default and JSON format|
| `c`| In sheet, copy the row to the clipboard|
| `t`| Tab switcher|
| `h j k l` or `← ↓ ↑ →`| Navigation |
| `b` / `w` | Previous / next column|
| `e` | Toggle Auto-Fit|
| `Ctrl + u` / `Ctrl + d`| Move half page up/down|
| `Ctrl + b` / `Ctrl + f`| Move full page up/down|
| `Home` or `g`| Move to first row|
| `End` or `G`| Move to last row|
| `Ctrl + r`| Reset data frame|
| `r`| Refresh the table from its source file|
| `q`| Close |
| `Q`| Quit Application |
| `:`| Command Palette|
| `/`| Fuzzy Search|

## Useful Commands

Press `:` to open the command palette and type to filter. It lists everything Tabiew can
do, including `Query`, `Filter`, `Order`, `Select`, `Cast`, `Import`, `Export`, `Schema`,
`ThemeSelector`, and `ToggleNerdFont`.

Four prefixes skip the list and open an inline editor directly:

|Prefix|Opens|
|-|-|
|`q `|SQL query|
|`s `|Select|
|`o `|Order|
|`f `|Filter|

## Configuration

Settings live in `~/.config/tabiew/config.toml` and are written back whenever you change
one from the command palette. Of note, `use_nerd_font` (default `false`) turns on the
glyphs in the status bar and pickers; leave it off unless your terminal uses a patched
font, or they render as empty boxes.

## Contributing

Contributions are welcome! Please fork the repository and submit pull requests with your features and bug fixes.

## Acknowledgments

This application uses themes from the [Ghostty terminal](https://ghostty.org/).


## License

This project is licensed under the MIT License - see the LICENSE file for details.
