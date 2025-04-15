#!/bin/bash
set -e

# Plugin name and version
PLUGIN_NAME="tarkov-price-overlay"
VERSION="0.1.0"

# Determine OS and set OBS plugin directory
case "$(uname -s)" in
    Darwin)
        # macOS
        OBS_PLUGIN_DIR="/Applications/OBS.app/Contents/PlugIns"
        PLUGIN_DIR="$OBS_PLUGIN_DIR/$PLUGIN_NAME.plugin/Contents/MacOS"
        ;;
    Linux)
        # Linux
        if [ -d "/usr/lib/obs-plugins" ]; then
            OBS_PLUGIN_DIR="/usr/lib/obs-plugins"
            PLUGIN_DIR="$OBS_PLUGIN_DIR/$PLUGIN_NAME/bin/64bit"
        elif [ -d "/usr/lib64/obs-plugins" ]; then
            OBS_PLUGIN_DIR="/usr/lib64/obs-plugins"
            PLUGIN_DIR="$OBS_PLUGIN_DIR/$PLUGIN_NAME/bin/64bit"
        else
            echo "OBS plugin directory not found. Please install OBS Studio first."
            exit 1
        fi
        ;;
    MINGW*|MSYS*|CYGWIN*)
        # Windows
        if [ -d "/c/Program Files/obs-studio" ]; then
            OBS_PLUGIN_DIR="/c/Program Files/obs-studio/obs-plugins/64bit"
            PLUGIN_DIR="$OBS_PLUGIN_DIR/$PLUGIN_NAME"
        elif [ -d "/c/Program Files (x86)/obs-studio" ]; then
            OBS_PLUGIN_DIR="/c/Program Files (x86)/obs-studio/obs-plugins/64bit"
            PLUGIN_DIR="$OBS_PLUGIN_DIR/$PLUGIN_NAME"
        else
            echo "OBS plugin directory not found. Please install OBS Studio first."
            exit 1
        fi
        ;;
    *)
        echo "Unsupported operating system: $(uname -s)"
        exit 1
        ;;
esac

# Create plugin directory
echo "Creating plugin directory: $PLUGIN_DIR"
mkdir -p "$PLUGIN_DIR"

# Copy the plugin library
echo "Copying plugin library..."
case "$(uname -s)" in
    Darwin)
        cp "target/release/lib$PLUGIN_NAME.dylib" "$PLUGIN_DIR/"
        ;;
    Linux)
        cp "target/release/lib$PLUGIN_NAME.so" "$PLUGIN_DIR/"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        cp "target/release/$PLUGIN_NAME.dll" "$PLUGIN_DIR/"
        ;;
esac

# Copy data directory
echo "Copying data directory..."
case "$(uname -s)" in
    Darwin)
        sudo mkdir -p "$OBS_PLUGIN_DIR/$PLUGIN_NAME/data"
        sudo cp -r data "$OBS_PLUGIN_DIR/$PLUGIN_NAME/"
        ;;
    Linux)
        sudo mkdir -p "$OBS_PLUGIN_DIR/$PLUGIN_NAME/data"
        sudo cp -r data "$OBS_PLUGIN_DIR/$PLUGIN_NAME/"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        mkdir -p "$OBS_PLUGIN_DIR/$PLUGIN_NAME/data"
        cp -r data "$OBS_PLUGIN_DIR/$PLUGIN_NAME/"
        ;;
esac

# Create Info.plist for macOS
if [ "$(uname -s)" = "Darwin" ]; then
    echo "Creating Info.plist for macOS..."
    sudo mkdir -p "$OBS_PLUGIN_DIR/$PLUGIN_NAME.plugin/Contents"
    sudo bash -c "cat > '$OBS_PLUGIN_DIR/$PLUGIN_NAME.plugin/Contents/Info.plist'" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>lib$PLUGIN_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>com.$PLUGIN_NAME.obs-plugin</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Tarkov Price Overlay</string>
    <key>CFBundlePackageType</key>
    <string>BNDL</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2023 Tarkov Price Overlay Developers</string>
</dict>
</plist>
EOF
fi

echo "Installation complete! Please restart OBS Studio." 