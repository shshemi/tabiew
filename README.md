## Tabiew

The simple template will create the following project structure:

Tabiew is a lightweight, terminal-based application designed to help view and query CSV files directly in your terminal. It utilizes a Polars dataframes and its querying engine to perform complex data manipulations without the need for a database system.

![Image Alt text](/images/screenshot.png "Screenshot")

## Features

- **View** CSV files directly in your terminal.
- **SQL Query Support**: Execute SQL queries on CSV data to filter, sort, and analyze the data.

## Installation

Since Tabiew is in its early stages, it is not published in any package manager and needs to be built from source.

First make sure you have 1.80.0-nightly version of Rust installed.

```bash
git pull https://github.com/shshemi/tabiew.git
cd tabiew
rustup override set nightly
cargo build --release
cp /target/release/tabiew <system_or_local_bin_path>
```