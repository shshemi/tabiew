#!/bin/sh
# Install script for Tabiew (tw)
# Usage: curl -sS https://raw.githubusercontent.com/shshemi/tabiew/main/install.sh | sh

set -eu

REPO="shshemi/tabiew"
BINARY_NAME="tw"
GITHUB_BASE="https://github.com/${REPO}"

err() {
    printf "error: %s\n" "$1" >&2
    exit 1
}

info() {
    printf "  %s\n" "$1"
}

need_cmd() {
    if ! command -v "$1" > /dev/null 2>&1; then
        err "need '$1' (command not found)"
    fi
}

check_cmd() {
    command -v "$1" > /dev/null 2>&1
}

detect_target() {
    _os="$(uname -s)"
    _arch="$(uname -m)"

    case "$_os" in
        Linux)
            case "$_arch" in
                x86_64)  echo "x86_64-unknown-linux-gnu" ;;
                aarch64) echo "aarch64-unknown-linux-gnu" ;;
                arm64)   echo "aarch64-unknown-linux-gnu" ;;
                armv7l)  echo "armv7-unknown-linux-gnueabihf" ;;
                *)       err "unsupported architecture: $_arch (try: cargo install --locked tabiew)" ;;
            esac
            ;;
        Darwin)
            case "$_arch" in
                x86_64)  echo "x86_64-apple-darwin" ;;
                arm64)   echo "aarch64-apple-darwin" ;;
                aarch64) echo "aarch64-apple-darwin" ;;
                *)       err "unsupported architecture: $_arch (try: cargo install --locked tabiew)" ;;
            esac
            ;;
        *)
            err "unsupported OS: $_os (try: cargo install --locked tabiew)"
            ;;
    esac
}

get_latest_version() {
    _url="${GITHUB_BASE}/releases/latest"
    if check_cmd curl; then
        _response="$(curl -fsS -o /dev/null -w '%{redirect_url}' "$_url" 2>/dev/null)" || \
            err "failed to determine latest version (could not reach GitHub)"
    elif check_cmd wget; then
        _response="$(wget --max-redirect=0 -q -O /dev/null --server-response "$_url" 2>&1 | \
            sed -n 's/.*Location: *//p' | head -1)" || true
        [ -z "$_response" ] && err "failed to determine latest version (could not reach GitHub)"
    else
        err "need curl or wget to download"
    fi
    [ -z "$_response" ] && err "failed to determine latest version (empty redirect)"
    # Extract tag from redirect URL: .../releases/tag/v0.12.0 -> v0.12.0
    echo "$_response" | sed 's|.*/||'
}

download() {
    _url="$1"
    _output="$2"
    if check_cmd curl; then
        curl -fsSL -o "$_output" "$_url"
    elif check_cmd wget; then
        wget -qO "$_output" "$_url"
    else
        err "need curl or wget to download"
    fi
}

usage() {
    cat <<EOF
install.sh — install Tabiew (tw)

Usage:
    install.sh [OPTIONS]

Options:
    -d, --dir DIR       Install binary to DIR (default: ~/.local/bin)
    -V, --version VER   Install a specific version (e.g. v0.12.0)
    -h, --help          Show this help message
EOF
}

main() {
    _version=""
    _dir=""

    while [ $# -gt 0 ]; do
        case "$1" in
            -h|--help)
                usage
                exit 0
                ;;
            -V|--version)
                [ $# -lt 2 ] && err "--version requires a value"
                _version="$2"
                shift
                ;;
            -d|--dir)
                [ $# -lt 2 ] && err "--dir requires a value"
                _dir="$2"
                shift
                ;;
            *)
                err "unknown option: $1 (see --help)"
                ;;
        esac
        shift
    done

    need_cmd uname
    need_cmd chmod

    _target="$(detect_target)"
    info "detected target: $_target"

    if [ -z "$_version" ]; then
        info "fetching latest version..."
        _version="$(get_latest_version)"
    fi
    [ -z "$_version" ] && err "could not determine version"
    info "version: $_version"

    # Determine install directory
    if [ -n "$_dir" ]; then
        _install_dir="$_dir"
    elif [ -d "/usr/local/bin" ] && [ -w "/usr/local/bin" ]; then
        _install_dir="/usr/local/bin"
    else
        _install_dir="${HOME}/.local/bin"
    fi
    mkdir -p "$_install_dir"

    _prefix="${HOME}/.local"

    # Download binary
    _bin_url="${GITHUB_BASE}/releases/download/${_version}/${BINARY_NAME}-${_target}"
    _tmp_bin="$(mktemp)"
    info "downloading ${BINARY_NAME}..."
    download "$_bin_url" "$_tmp_bin" || err "failed to download binary from ${_bin_url}"
    chmod +x "$_tmp_bin"
    mv "$_tmp_bin" "${_install_dir}/${BINARY_NAME}"
    info "installed ${BINARY_NAME} to ${_install_dir}/${BINARY_NAME}"

    # Download and install man pages + completions
    _extras_url="${GITHUB_BASE}/releases/download/${_version}/tabiew-manual-and-completions.tar.gz"
    _tmp_extras="$(mktemp)"
    if download "$_extras_url" "$_tmp_extras" 2>/dev/null; then
        _tmp_dir="$(mktemp -d)"
        tar xzf "$_tmp_extras" -C "$_tmp_dir" 2>/dev/null || {
            info "warning: failed to extract extras archive, skipping man/completions"
            rm -rf "$_tmp_extras" "$_tmp_dir"
        }

        if [ -d "$_tmp_dir" ]; then
            # Man pages
            if [ -f "$_tmp_dir/manual/tabiew.1" ]; then
                _man_dir="${_prefix}/share/man/man1"
                mkdir -p "$_man_dir"
                cp "$_tmp_dir/manual/tabiew.1" "${_man_dir}/tabiew.1"
                cp "$_tmp_dir/manual/tabiew.1" "${_man_dir}/tw.1"
                info "installed man pages to ${_man_dir}"
            fi

            # Bash completion
            if [ -f "$_tmp_dir/completion/tw.bash" ]; then
                _bash_dir="${_prefix}/share/bash-completion/completions"
                mkdir -p "$_bash_dir"
                cp "$_tmp_dir/completion/tw.bash" "${_bash_dir}/tw.bash"
                info "installed bash completion"
            fi

            # Zsh completion
            if [ -f "$_tmp_dir/completion/_tw" ]; then
                _zsh_dir="${_prefix}/share/zsh/site-functions"
                mkdir -p "$_zsh_dir"
                cp "$_tmp_dir/completion/_tw" "${_zsh_dir}/_tw"
                info "installed zsh completion"
            fi

            # Fish completion
            if [ -f "$_tmp_dir/completion/tw.fish" ]; then
                _fish_dir="${_prefix}/share/fish/completions"
                mkdir -p "$_fish_dir"
                cp "$_tmp_dir/completion/tw.fish" "${_fish_dir}/tw.fish"
                info "installed fish completion"
            fi

            rm -rf "$_tmp_dir"
        fi
        rm -f "$_tmp_extras"
    else
        info "warning: could not download extras (man pages/completions), skipping"
        rm -f "$_tmp_extras"
    fi

    # Check if install dir is on PATH
    case ":${PATH}:" in
        *":${_install_dir}:"*)
            ;;
        *)
            printf "\n"
            info "WARNING: ${_install_dir} is not on your \$PATH"
            info "Add it by running:"
            info "  export PATH=\"${_install_dir}:\$PATH\""
            info "Or add the above line to your shell profile (~/.bashrc, ~/.zshrc, etc.)"
            ;;
    esac

    printf "\n"
    info "Tabiew (tw) ${_version} installed successfully!"
}

{
    main "$@"
}
