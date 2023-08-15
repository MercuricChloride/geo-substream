#!/usr/bin/env sh

# takes two arguments: the endpoint and the address of the contract
# e.g. ./deploy.sh http://localhost:8545 0x1234567890123456789012345678901234567890
GRAPH_ENDPOINT=$1
CONTRACT_ADDRESS=$2
DEFAULT_GEO_ADDRESS="0x170b749413328ac9a94762031a7A05b00c1D2e34"

if [ -z "$GRAPH_ENDPOINT" ]
then
  echo "Graph endpoint not supplied.\n USEAGE: ./deploy.sh <GRAPH_ENDPOINT> <CONTRACT_ADDRESS>"
  exit 1
fi

if [ -z "$CONTRACT_ADDRESS" ]
then
  echo "Contract address not supplied.\n USEAGE: ./deploy.sh <GRAPH_ENDPOINT> <CONTRACT_ADDRESS>"
  exit 1
fi

# Replace the DEFAULT_GEO_ADDRESS in src/lib.rs with the address of the contract
sed -i '' "s/$DEFAULT_GEO_ADDRESS/$CONTRACT_ADDRESS/g" src/lib.rs

# Build the WASM file
cargo build --target wasm32-unknown-unknown --release

# Pack the spkg
substreams pack -o geo-substream.spkg

# Deploy the subgraph
graph deploy --product hosted-service $GRAPH_ENDPOINT
