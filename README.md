# Tarkov Price Overlay

A Rust plugin for OBS Studio that displays Tarkov market prices.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [OBS Studio](https://obsproject.com/) (latest version)
- [pre-commit](https://pre-commit.com/) (for code formatting)

## Installation

### From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/tarkov-price-overlay.git
   cd tarkov-price-overlay
   ```

2. Install pre-commit hooks:
   ```bash
   ./scripts/install-pre-commit.sh
   ```

3. Build the plugin:
   ```bash
   cargo build --release
   ```

4. Install the plugin:
   ```bash
   ./install_to_obs.sh
   ```

### From Release

Download the appropriate package for your platform from the [releases page](https://github.com/yourusername/tarkov-price-overlay/releases) and follow the installation instructions in the package.

## Usage

1. Open OBS Studio
2. Add a new Browser Source
3. Set the URL to `http://localhost:8080`
4. Configure the source size and position as needed

## Development

### Code Formatting

This project uses pre-commit hooks to ensure consistent code formatting. The hooks are automatically installed when you run `./scripts/install-pre-commit.sh`. They will run automatically before each commit, but you can also run them manually:

```bash
pre-commit run --all-files
```

### Building

To build the plugin in debug mode:

```bash
cargo build
```

To build in release mode:

```bash
cargo build --release
```

### Testing

To run the tests:

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Features

- **Real-time item detection**: Identifies items in your game using computer vision
- **Accurate pricing**: Fetches up-to-date prices from the Tarkov Market API
- **Customizable overlay**: Configure colors, fonts, and display options
- **Minimum value threshold**: Only show prices for items above a certain value
- **Performance optimized**: Minimal impact on game and streaming performance
- **Automatic template downloading**: One-click download of all item templates from the API

## Configuration

The plugin provides several configuration options:

- **API Key**: Your Tarkov Market API key
- **Minimum Value Threshold**: Only display items worth more than this amount (in roubles)
- **Detection Threshold**: Controls the sensitivity of item detection (0.0-1.0)
- **Download Item Templates**: Downloads all item icons from the Tarkov Market API
- **Highlight Enabled**: Toggle whether to highlight detected items
- **Tooltip Enabled**: Toggle whether to show price tooltips
- **Highlight Color**: Color for item highlighting
- **Tooltip Font Size**: Size of the price tooltip text
- **Tooltip Font Color**: Color for the price tooltip text

## Template Management

The plugin uses template matching to detect items. Templates are stored in the `data/templates` directory. Each template is a PNG image named after the item's unique ID from the Tarkov Market API.

### Automatic Template Download

The plugin provides an automated way to download all item templates:

1. Enter your API key in the settings
2. Click the "Download Item Templates" button
3. Wait for the download to complete (this may take a few minutes)
4. The templates will be automatically loaded and ready to use

### Custom Templates

If you want to add your own custom templates:

1. Take a screenshot of the item in-game
2. Crop the image tightly around the item icon
3. Convert to a PNG file and name it with the item's UID (e.g., `5c0e531d86f7747fa23f4d42.png`)
4. Place the image in the `data/templates` directory
5. Restart the plugin or reload OBS

## Troubleshooting

- **No items detected**: Try adjusting the Detection Threshold, or check that your templates are correctly formatted
- **Wrong prices displayed**: Make sure your API key is valid and that you have a stable internet connection
- **Plugin crashes**: Check the OBS log for details and report the issue on GitHub
- **Templates not downloading**: Verify your internet connection and API key, then try again

## Building with Rust

This project has been partially converted from C/C++ to Rust. The Rust code compiles to a shared library that interfaces with OBS.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (comes with Rust)

### Building Locally

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

The output library will be in `target/debug/` or `target/release/` depending on your build type.

### GitHub Actions

The project uses GitHub Actions for CI/CD:

1. **Rust Build Workflow**: Builds the Rust code for multiple platforms (Linux and macOS, both x86_64 and arm64 for macOS).
2. **Format Check**: Ensures code formatting matches the project's standards using rustfmt.
3. **Release Process**: Automatically builds and packages the project when a tag is pushed.

#### Running Tests

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Tarkov Market](https://tarkov-market.app/) for providing the item price API
- [OpenCV](https://opencv.org/) for computer vision algorithms
- [OBS Studio](https://obsproject.com/) for the streaming platform
