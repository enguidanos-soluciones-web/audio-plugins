#!/usr/bin/env bash
# macOS PKG installer builder for Take My Headphones CLAP plugin
#
# Usage:
#   macos.sh --bundle    <path/to/plugin.clap>
#            --id        com.example.Plugin
#            --version   1.0.0
#            --out       /path/to/output.pkg
#
# Requires: pkgbuild, productbuild (included in Xcode Command Line Tools)

set -euo pipefail

BUNDLE=""
BUNDLE_ID=""
VERSION=""
OUTFILE=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --bundle)  BUNDLE="$2";    shift 2 ;;
        --id)      BUNDLE_ID="$2"; shift 2 ;;
        --version) VERSION="$2";   shift 2 ;;
        --out)     OUTFILE="$2";   shift 2 ;;
        *) echo "Unknown argument: $1" >&2; exit 1 ;;
    esac
done

if [[ -z "$BUNDLE" || -z "$BUNDLE_ID" || -z "$VERSION" || -z "$OUTFILE" ]]; then
    echo "Usage: $0 --bundle <plugin.clap> --id <bundle-id> --version <x.y.z> --out <output.pkg>" >&2
    exit 1
fi

COMPONENT_PKG="$(mktemp -d)/component.pkg"

pkgbuild \
    --component "$BUNDLE" \
    --install-location "/Library/Audio/Plug-Ins/CLAP" \
    --identifier "$BUNDLE_ID" \
    --version "$VERSION" \
    "$COMPONENT_PKG"

productbuild \
    --package "$COMPONENT_PKG" \
    --identifier "${BUNDLE_ID}.installer" \
    --version "$VERSION" \
    "$OUTFILE"

echo "PKG installer: $OUTFILE"
