#!/bin/bash

# ACBU Soroban Contracts Deployment Script
# Usage: ./deploy.sh [testnet|mainnet]

set -e

NETWORK=${1:-testnet}
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONTRACTS_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Deploying ACBU contracts to ${NETWORK}${NC}"

# Check if soroban CLI is installed
if ! command -v soroban &> /dev/null; then
    echo -e "${RED}Error: soroban CLI not found. Please install it first:${NC}"
    echo "cargo install --locked soroban-cli"
    exit 1
fi

# Set network
if [ "$NETWORK" = "testnet" ]; then
    NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
    HORIZON_URL="https://horizon-testnet.stellar.org"
    FRIENDBOT_URL="https://friendbot.stellar.org"
elif [ "$NETWORK" = "mainnet" ]; then
    NETWORK_PASSPHRASE="Public Global Stellar Network ; September 2015"
    HORIZON_URL="https://horizon.stellar.org"
    FRIENDBOT_URL=""
    
    echo -e "${YELLOW}Warning: Deploying to MAINNET. Make sure you have:${NC}"
    echo "1. Tested on testnet"
    echo "2. Security audit completed"
    echo "3. Backup of secret keys"
    read -p "Continue? (yes/no): " confirm
    if [ "$confirm" != "yes" ]; then
        echo "Deployment cancelled"
        exit 1
    fi
else
    echo -e "${RED}Error: Invalid network. Use 'testnet' or 'mainnet'${NC}"
    exit 1
fi

# Check for secret key
if [ -z "$STELLAR_SECRET_KEY" ]; then
    echo -e "${RED}Error: STELLAR_SECRET_KEY environment variable not set${NC}"
    exit 1
fi

# Build contracts
echo -e "${GREEN}Building contracts...${NC}"
cd "$CONTRACTS_DIR"
cargo build --target wasm32-unknown-unknown --release

# Deploy contracts in order: Oracle -> Reserve Tracker -> Minting -> Burning
echo -e "${GREEN}Deploying contracts...${NC}"

# Deploy Oracle
echo -e "${YELLOW}Deploying Oracle contract...${NC}"
ORACLE_WASM="$CONTRACTS_DIR/target/wasm32-unknown-unknown/release/oracle.wasm"
ORACLE_ID=$(soroban contract deploy \
    --wasm "$ORACLE_WASM" \
    --network "$NETWORK" \
    --source "$STELLAR_SECRET_KEY" \
    | grep -oP 'Contract ID: \K[^\s]+')

echo -e "${GREEN}Oracle deployed: $ORACLE_ID${NC}"

# Deploy Reserve Tracker
echo -e "${YELLOW}Deploying Reserve Tracker contract...${NC}"
RESERVE_WASM="$CONTRACTS_DIR/target/wasm32-unknown-unknown/release/reserve_tracker.wasm"
RESERVE_ID=$(soroban contract deploy \
    --wasm "$RESERVE_WASM" \
    --network "$NETWORK" \
    --source "$STELLAR_SECRET_KEY" \
    | grep -oP 'Contract ID: \K[^\s]+')

echo -e "${GREEN}Reserve Tracker deployed: $RESERVE_ID${NC}"

# Deploy Minting
echo -e "${YELLOW}Deploying Minting contract...${NC}"
MINTING_WASM="$CONTRACTS_DIR/target/wasm32-unknown-unknown/release/minting.wasm"
MINTING_ID=$(soroban contract deploy \
    --wasm "$MINTING_WASM" \
    --network "$NETWORK" \
    --source "$STELLAR_SECRET_KEY" \
    | grep -oP 'Contract ID: \K[^\s]+')

echo -e "${GREEN}Minting deployed: $MINTING_ID${NC}"

# Deploy Burning
echo -e "${YELLOW}Deploying Burning contract...${NC}"
BURNING_WASM="$CONTRACTS_DIR/target/wasm32-unknown-unknown/release/burning.wasm"
BURNING_ID=$(soroban contract deploy \
    --wasm "$BURNING_WASM" \
    --network "$NETWORK" \
    --source "$STELLAR_SECRET_KEY" \
    | grep -oP 'Contract ID: \K[^\s]+')

echo -e "${GREEN}Burning deployed: $BURNING_ID${NC}"

# Save contract addresses
DEPLOYMENT_FILE="$CONTRACTS_DIR/.soroban/deployment_${NETWORK}.json"
mkdir -p "$(dirname "$DEPLOYMENT_FILE")"
cat > "$DEPLOYMENT_FILE" << EOF
{
  "network": "$NETWORK",
  "deployed_at": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "contracts": {
    "oracle": "$ORACLE_ID",
    "reserve_tracker": "$RESERVE_ID",
    "minting": "$MINTING_ID",
    "burning": "$BURNING_ID"
  }
}
EOF

echo -e "${GREEN}Deployment complete!${NC}"
echo -e "${GREEN}Contract addresses saved to: $DEPLOYMENT_FILE${NC}"
echo ""
echo "Contract Addresses:"
echo "  Oracle: $ORACLE_ID"
echo "  Reserve Tracker: $RESERVE_ID"
echo "  Minting: $MINTING_ID"
echo "  Burning: $BURNING_ID"
