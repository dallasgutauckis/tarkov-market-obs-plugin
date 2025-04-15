#!/bin/bash

# Test script for the Tarkov Price Overlay plugin mock

# Compile the test plugin
echo "Compiling test plugin..."
gcc -o test_plugin test_plugin.c

# Run the test
echo "Running tests..."
./test_plugin

# Check the result
if [ $? -eq 0 ]; then
    echo "All tests passed!"
else
    echo "Tests failed!"
    exit 1
fi

# Cleanup
echo "Cleaning up..."
rm -f test_plugin

echo "Test script completed successfully!" 