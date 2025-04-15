#!/bin/bash

# Install pre-commit if not already installed
if ! command -v pre-commit &> /dev/null; then
    echo "Installing pre-commit..."
    pip install pre-commit
fi

# Install the hooks
echo "Installing pre-commit hooks..."
pre-commit install

# Run pre-commit on all files
echo "Running pre-commit on all files..."
pre-commit run --all-files 