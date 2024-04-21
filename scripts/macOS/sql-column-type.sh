#!/bin/bash

types=("INT" "VARCHAR(255)" "TEXT" "DATE" "FLOAT" "BOOLEAN")
printf "\e[36mSelect the data type for the column:\e[0m 📊\n"
for i in "${!types[@]}"; do
    printf "\e[32m$((i + 1))) ${types[i]}\e[0m\n"
done

select_column_type() {
    read -p ": " choice
    if [[ "$choice" =~ ^[1-6]$ ]]; then
        printf "${types[$choice-1]}\n"
    else
        printf "\e[31mInvalid selection. Try again.\e[0m ❌\n"
        select_column_type
    fi
}

printf "\e[34mEnter your choice: \e[0m🔢 "
column_type=$(select_column_type)
printf "\e[35mSelected data type: $column_type\e[0m ✔️\n"

