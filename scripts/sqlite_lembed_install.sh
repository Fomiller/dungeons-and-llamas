#!/bin/sh
set -e

if [ -n "$NO_COLOR" ]; then
    BOLD=""
    RESET=""
else
    BOLD="\033[1m"
    RESET="\033[0m"
fi


usage() {
    cat <<EOF
sqlite-hello-install 0.0.1-alpha.8

USAGE:
    $0 [static|loadable] [--target=target] [--prefix=path]

OPTIONS:
    --target
            Specify a different target platform to install. Available targets: linux-x86_64, macos-aarch64, macos-x86_64, windows-x86_64

    --prefix
            Specify a different directory to save the binaries. Defaults to the current working directory.
EOF
}




current_target() {
  if [ "$OS" = "Windows_NT" ]; then
    # TODO disambiguate between x86 and arm windows
    target="windows-x86_64"
    return 0
  fi
  case $(uname -sm) in
  "Darwin x86_64") target=macos-x86_64 ;;
  "Darwin arm64") target=macos-aarch64 ;;
  "Linux x86_64") target=linux-x86_64 ;;
  *) target=$(uname -sm);;
  esac
}



process_arguments() {
  while [[ $# -gt 0 ]]; do
      case "$1" in
          --help)
              usage
              exit 0
              ;;
          --target=*)
              target="\${1#*=}"
              ;;
          --prefix=*)
              prefix="\${1#*=}"
              ;;
          static|loadable)
              type="$1"
              ;;
          *)
              echo "Unrecognized option: $1"
              usage
              exit 1
              ;;
      esac
      shift
  done
  if [ -z "$type" ]; then
    type=loadable
  fi
  if [ "$type" != "static" ] && [ "$type" != "loadable" ]; then
      echo "Invalid type '$type'. It must be either 'static' or 'loadable'."
      usage
      exit 1
  fi
  if [ -z "$prefix" ]; then
    prefix="$PWD"
  fi
  if [ -z "$target" ]; then
    current_target
  fi
}




main() {
    local type=""
    local target=""
    local prefix=""
    local url=""
    local checksum=""

    process_arguments "$@"

    echo "${BOLD}Type${RESET}: $type"
    echo "${BOLD}Target${RESET}: $target"
    echo "${BOLD}Prefix${RESET}: $prefix"

    case "$target-$type" in
    "windows-x86_64-loadable")
      url="https://github.com/asg017/sqlite-lembed/releases/download/v0.0.1-alpha.8/sqlite-lembed-0.0.1-alpha.8-loadable-windows-x86_64.tar.gz"
      checksum="b751c29ce5cf326c06e16259b760b43b75a8cede2af263b2a1a65bcfa27bb055"
      ;;
    "macos-aarch64-loadable")
      url="https://github.com/asg017/sqlite-lembed/releases/download/v0.0.1-alpha.8/sqlite-lembed-0.0.1-alpha.8-loadable-macos-aarch64.tar.gz"
      checksum="1ba6a2b5cc06e9f664bfdc01310ae0de3f3f9112015b694c9e035b2e840f0b87"
      ;;
    "linux-x86_64-loadable")
      url="https://github.com/asg017/sqlite-lembed/releases/download/v0.0.1-alpha.8/sqlite-lembed-0.0.1-alpha.8-loadable-linux-x86_64.tar.gz"
      checksum="934bea893d4e112fb2aa8e3bfac2fa216d0e67a1b4f143c79ed6528408406f0a"
      ;;
    "macos-x86_64-loadable")
      url="https://github.com/asg017/sqlite-lembed/releases/download/v0.0.1-alpha.8/sqlite-lembed-0.0.1-alpha.8-loadable-macos-x86_64.tar.gz"
      checksum="8e0669d772aca64e4ad5fc18ecbdb4afe95976a7a6fa0ca8d12f294eab72eb02"
      ;;
    *)
      echo "Unsupported platform $target" 1>&2
      exit 1
      ;;
    esac

    extension="\${url##*.}"

    if [ "$extension" = "zip" ]; then
      tmpfile="$prefix/tmp.zip"
    else
      tmpfile="$prefix/tmp.tar.gz"
    fi

    curl --fail --location --progress-bar --output "$tmpfile" "$url"

    if ! echo "$checksum $tmpfile" | sha256sum --check --status; then
      echo "Checksum fail!"  1>&2
      rm $tmpfile
      exit 1
    fi

    if [ "$extension" = "zip" ]; then
      unzip "$tmpfile" -d $prefix
      rm $tmpfile
    else
      tar -xzf "$tmpfile" -C $prefix
      rm $tmpfile
    fi

    echo "✅ $target $type binaries installed at $prefix."
}



main "$@"
