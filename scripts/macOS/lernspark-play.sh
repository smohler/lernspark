#!/bin/bash

# Store the absolute path of this script in a variable
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Navigate to ../../data
cd $DIR/../../data
cargo run

cd $DIR/../../pipeline

# Open the jupyter notebook
jupyter notebook sandbox.ipynb

cd $DIR
