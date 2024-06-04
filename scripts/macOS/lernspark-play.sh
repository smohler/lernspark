#!/bin/bash

# Store the absolute path of this script in a variable
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Navigate to ../../data
cd $DIR/../../data
cargo run

cd $DIR/../../pipeline

# Start a python virtual env for python3.11
python -m venv lernspark
source lernspark/bin/activate

# Open the notebook sandbox.ipynb in Jupyter and after it is closed, deactivate the virtual env
jupyter notebook sandbox.ipynb

# Deactivate the virtual env
deactivate

cd $DIR
