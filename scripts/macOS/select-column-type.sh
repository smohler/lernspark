#!/bin/bash

# Include common variables and cleanup procedures
source setup.sh

types=("INT" "VARCHAR(255)" "TEXT" "DATE" "FLOAT" "BOOLEAN")
echo "Select the data type for the column:"
for i in "${!types[@]}"; do
    echo "$((i + 1))) ${types[i]}"
done

select_column_type() {
    read -p "Enter your choice: " choice
    if [[ "$choice" =~ ^[1-6]$ ]]; then
        echo "${types[$choice-1]}"
    else
        echo "Invalid selection. Try again."
        select_column_type
    fi
}

column_type=$(select_column_type)
echo "Selected data type: $column_type"
