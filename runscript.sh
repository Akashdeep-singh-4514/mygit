#!/bin/bash

cargo build --release && mkdir -p sample &&
cp target/release/mygit sample/
