#!/bin/bash

cargo build --release
sudo cp -r target/release/anijouhou /usr/bin/