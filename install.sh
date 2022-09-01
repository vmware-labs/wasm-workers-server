#!/usr/bin/env sh
# Script to install Wasm Workers Server in your system

# Global
REPO="vmware-labs/wasm-workers-server"
GITHUB_URL="https://github.com/$REPO"
GITHUB_API_URL="https://api.github.com/repos/$REPO"
TMP_FOLDER="/tmp/wws"
TMP_FILE="/tmp/wws.tar.gz"
# This may change based on the --local argument
TOOL_LOCATION="/usr/local/bin/wws"
LOCAL_INSTALLATION=false

# Get the OS information
OS_TYPE=$(uname -s)
ARCH_TYPE=$(uname -m)

# Compose the final URL for GitHub
OS_SEGMENT=""
ARCH_SEGMENT=""
BINARY_URL=""

_not_supported_and_exit() {
    echo "Your current platform $OS_TYPE ($ARCH_TYPE) is not supported yet."
    echo "Please, open an issue in $GITHUB_URL/issues"
    exit 1
}

_windows_and_exit() {
    echo "For Windows, please download the server directly from our GitHub page:"
    echo "  => $GITHUB_URL/releases/latest"
    exit 1
}

# Check arguments
if [ "$1" = "--local" ]; then
    TOOL_LOCATION="$(pwd)/wws"
    LOCAL_INSTALLATION=true
fi

# Start
echo "👋 Hello"
echo "I'm going to install Wasm Workers Server in your system"
echo ""

# Compatibility matrix
if [ "$OS_TYPE" = "Linux" ] || [ "$OS_TYPE" = "linux" ]; then
    OS_SEGMENT="linux"
elif [ "$OS_TYPE" = "Darwin" ] || [ "$OS_TYPE" = "darwin" ]; then
    OS_SEGMENT="darwin"
elif [ "$OS_TYPE" = "Windows" ] || [ "$OS_TYPE" = "windows" ]; then
    _windows_and_exit
else
    _not_supported_and_exit
fi

# Check for different architectures
if [ "$ARCH_TYPE" = "arm64" ] || [ "$ARCH_TYPE" = "aarch64" ]; then
    ARCH_SEGMENT="aarch64"
elif [ "$ARCH_TYPE" = "x86_64" ] || [ "$ARCH_TYPE" = "x64" ] || [ "$ARCH_TYPE" = "amd64" ]; then
    ARCH_SEGMENT="x86_64"
elif [ "$ARCH_TYPE" = "i386" ] || [ "$ARCH_TYPE" = "i686" ] || [ "$ARCH_TYPE" = "x86" ]; then
    ARCH_SEGMENT="x86_32"
else
    _not_supported_and_exit
fi

# Get the URL
URL=$(curl -v $GITHUB_API_URL/releases/latest 2>&1 | grep -v ant | grep browser_download_url | grep "$OS_SEGMENT" | grep "$ARCH_SEGMENT" | cut -d '"' -f 4)

if [ "$URL" = "" ]; then
    _not_supported_and_exit
fi

# Install!
echo "⚙️  Downloading"
curl  -L -o $TMP_FILE $URL

echo "⚙️  Decompressing"
mkdir -p $TMP_FOLDER
tar xvf $TMP_FILE -C $TMP_FOLDER

echo "⚙️  Installing"
if [ $LOCAL_INSTALLATION = false ] && [ "$OS_TYPE" = "Darwin" ] || [ "$OS_TYPE" = "darwin" ]; then
    echo "Wasm Workers Server will be installed in /usr/local/bin."
    echo "This requires sudo permissions. If you prefer to install it"
    echo "in your current directory, run the installer with --local."
    echo "If you want it to be global, just type your password:"

    sudo mv $TMP_FOLDER/wasm-worksers-server-*/wws $TOOL_LOCATION
    sudo chmod +x $TOOL_LOCATION
else
    mv $TMP_FOLDER/wasm-worksers-server-*/wws $TOOL_LOCATION
    chmod +x $TOOL_LOCATION
fi

echo "🧹 Cleaning up"
rm -r $TMP_FILE $TMP_FOLDER

echo "🚀 Wasm Workers Server (wws) was installed correctly!"

if [ $LOCAL_INSTALLATION = true ]; then
    echo "You can now try it: ./wws --help"
else
    echo "You can now try it: wws --help"
fi