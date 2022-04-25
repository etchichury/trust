#!/bin/bash
export CARGO_TARGET_DIR=~/projects/trust/target
cargo build --release
sudo setcap cap_net_admin=eip $CARGO_TARGET_DIR/release/trust
$CARGO_TARGET_DIR/release/trust & # creates tun0 device 
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0 #set ip for the device
sudo ip link set up dev tun0 #change device state
trap "kill $pid" INT TERM
wait $pid