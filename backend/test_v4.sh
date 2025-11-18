#!/bin/bash
# Quick test of V4 engine with null move pruning

cat <<EOF | timeout 10 cargo run --release --bin uci
setoption name Engine value V4
position startpos
go depth 5
quit
EOF
