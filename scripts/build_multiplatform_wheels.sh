#!/bin/bash
set -e

# Multi-platform wheel building script for namedivider-rust
# This script builds wheels for multiple Python versions and platforms
# NOTE: Requires family_names.txt in namedivider-rs/ directory

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
PYTHON_DIR="$PROJECT_ROOT/python"
DIST_DIR="$PROJECT_ROOT/dist"

# Check for family_names.txt
if [ ! -f "$PROJECT_ROOT/namedivider-rs/family_names.txt" ]; then
    echo "Error: family_names.txt not found in namedivider-rs/ directory"
    echo "Please place family_names.txt before building"
    exit 1
fi

# Create dist directory
mkdir -p "$DIST_DIR"

# Python versions to build for
PYTHON_VERSIONS=("3.9" "3.10" "3.11" "3.12" "3.13")

echo "Building wheels for namedivider-rust v0.3.0"
echo "Project root: $PROJECT_ROOT"
echo "Python directory: $PYTHON_DIR"
echo "Distribution directory: $DIST_DIR"

cd "$PYTHON_DIR"

# Build for each Python version
for py_version in "${PYTHON_VERSIONS[@]}"; do
    echo ""
    echo "=== Building for Python $py_version ==="
    
    # Check if Python version is available
    if ! command -v "python$py_version" &> /dev/null; then
        echo "Warning: Python $py_version not found, skipping..."
        continue
    fi
    
    # Build wheel
    echo "Building wheel with Python $py_version..."
    if python$py_version -m pip install maturin &> /dev/null; then
        python$py_version -m maturin build --release --out "$DIST_DIR"
        echo "✓ Successfully built wheel for Python $py_version"
    else
        echo "✗ Failed to install maturin for Python $py_version"
    fi
done

# List built wheels
echo ""
echo "=== Built wheels ==="
ls -la "$DIST_DIR"/*.whl 2>/dev/null || echo "No wheels found in $DIST_DIR"

echo ""
echo "Build complete! Wheels are available in: $DIST_DIR"
echo ""
echo "To test a wheel:"
echo "  python -m pip install --force-reinstall dist/namedivider_rust-*.whl"
echo "  python -c \"import namedivider_rust; print(namedivider_rust.BasicNameDivider().divide_name('田中太郎'))\""