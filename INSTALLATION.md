# Installation Guide

This guide explains how to install the Tarkov Price Overlay plugin for OBS Studio.

## Prerequisites

- OBS Studio 28.0 or later
- macOS 10.13 or later (for macOS users)
- Linux with glibc 2.17 or later (for Linux users)

## Installing from GitHub Releases

1. Go to the [Releases page](https://github.com/dallasgutauckis/tarkov-market-obs-plugin/releases)
2. Download the appropriate package for your system:
   - For Intel Macs: `tarkov-price-overlay-{version}-x86_64-apple-darwin.tar.gz`
   - For Apple Silicon Macs: `tarkov-price-overlay-{version}-aarch64-apple-darwin.tar.gz`
   - For Linux: `tarkov-price-overlay-{version}-x86_64-unknown-linux-gnu.tar.gz`

### macOS Installation

1. Extract the downloaded package:
   ```bash
   tar -xzf tarkov-price-overlay-{version}-x86_64-apple-darwin.tar.gz
   ```

2. Create the plugin directory:
   ```bash
   mkdir -p ~/Library/Application\ Support/obs-studio/plugins/tarkov-price-overlay/bin
   ```

3. Copy the plugin library:
   ```bash
   cp libtarkov_price_overlay.dylib ~/Library/Application\ Support/obs-studio/plugins/tarkov-price-overlay/bin/
   ```

4. Create the data directory:
   ```bash
   mkdir -p ~/Library/Application\ Support/obs-studio/plugins/tarkov-price-overlay/data
   ```

5. Restart OBS Studio

### Linux Installation

1. Extract the downloaded package:
   ```bash
   tar -xzf tarkov-price-overlay-{version}-x86_64-unknown-linux-gnu.tar.gz
   ```

2. Create the plugin directory:
   ```bash
   mkdir -p ~/.config/obs-studio/plugins/tarkov-price-overlay/bin/64bit
   ```

3. Copy the plugin library:
   ```bash
   cp libtarkov_price_overlay.so ~/.config/obs-studio/plugins/tarkov-price-overlay/bin/64bit/
   ```

4. Create the data directory:
   ```bash
   mkdir -p ~/.config/obs-studio/plugins/tarkov-price-overlay/data
   ```

5. Restart OBS Studio

## Verifying Installation

1. Open OBS Studio
2. Go to "Tools" > "Scripts"
3. Look for "Tarkov Item Price Overlay" in the list of available sources
4. You can also check the OBS log file:
   - macOS: `~/Library/Application Support/obs-studio/logs/`
   - Linux: `~/.config/obs-studio/logs/`

## Troubleshooting

If the plugin doesn't appear in OBS Studio:

1. Check the OBS log file for any error messages
2. Verify that the plugin files are in the correct locations
3. Make sure you have the correct version for your system architecture
4. Try restarting OBS Studio

## Building from Source

If you prefer to build the plugin from source, see the [Development Guide](DEVELOPMENT.md) for instructions.

## Support

If you encounter any issues, please:
1. Check the [Troubleshooting](#troubleshooting) section
2. Search the [GitHub Issues](https://github.com/dallasgutauckis/tarkov-market-obs-plugin/issues)
3. Create a new issue if your problem hasn't been reported 