#!/bin/sh
# Aura Language Toolchain Automated Installer (Linux & macOS Architecture Support)
set -e

echo "🌌 Initializing Aura Compiler Global Installation Pipeline..."

# 1. Define local target directory installation boundaries
AURA_HOME="$HOME/.aura"
AURA_BIN="$AURA_HOME/bin"

mkdir -p "$AURA_BIN"

# 2. Build the production-ready optimized compiler executable binary via Cargo
echo "⚙️  Compiling production-grade release assets utilizing Cargo..."
cargo build --release

# 3. Transfer the compiled machine binary asset into the local environment folder
echo "📦 Distributing toolchain binary executable into workspace tracks..."
cp target/release/aura "$AURA_BIN/aura"
chmod +x "$AURA_BIN/aura"

# 4. Detect the active user shell to apply the correct path profile configuration
DETECTED_PROFILE=""
if [ -n "$ZSH_VERSION" ]; then
    DETECTED_PROFILE="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    DETECTED_PROFILE="$HOME/.bashrc"
else
    # Fallback to standard POSIX login profile maps
    if [ -f "$HOME/.zshrc" ]; then
        DETECTED_PROFILE="$HOME/.zshrc"
    elif [ -f "$HOME/.bashrc" ]; then
        DETECTED_PROFILE="$HOME/.bashrc"
    else
        DETECTED_PROFILE="$HOME/.profile"
    fi
fi

echo "🔍 Detected active system user shell configuration target: $DETECTED_PROFILE"

# 5. Append the binary path to the user profile if it isn't already registered
PATH_EXPORT_COMMAND="export PATH=\"\$HOME/.aura/bin:\$PATH\""

if ! grep -q ".aura/bin" "$DETECTED_PROFILE" 2>/dev/null; then
    echo "Updating environment variable search paths inside $DETECTED_PROFILE..."
    echo "" >> "$DETECTED_PROFILE"
    echo "# Aura Language Toolchain System Path Binaries Pointer" >> "$DETECTED_PROFILE"
    echo "$PATH_EXPORT_COMMAND" >> "$DETECTED_PROFILE"
    echo "✓ Path configuration updated successfully."
else
    echo "✓ Aura binary path is already registered inside your environment profile."
fi

echo ""
echo "🎉 SUCCESS! Aura Language toolchain is now globally installed on your system!"
echo "👉 To apply the changes immediately, run: source $DETECTED_PROFILE"
echo "🚀 From then on, you can compile any file from any directory by simply typing: aura [file.au]"
echo ""
