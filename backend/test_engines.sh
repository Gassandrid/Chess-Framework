#!/bin/bash
# Quick test of V4 and MCTS engines

cat <<EOF | timeout 10 cargo run --release --bin uci
position startpos
go depth 3
quit
EOF
