#!/bin/bash
PWD=$(pwd)
SCRIPT_PATH=$(readlink -f "$0")
ROOT_DIR="$(dirname $(dirname $(dirname "$SCRIPT_PATH")))"
SCRIPT_DIR=$(dirname "$SCRIPT_PATH")
cd $SCRIPT_DIR

# Setup and cleanup configurations
source sql-setup.sh
printf "\e[32mWriting to ${SQL_FILE}\e[0m 📝\n"

# Loop for adding tables
while true; do
    printf "\e[36mDo you want to add a new table? (yes/no):\e[0m 🤔\n"
    read answer
    if [[ "$answer" != "yes" ]]; then
        printf "\e[33mFinished creating SQL schema.\e[0m ✅\n"
        break
    fi

    printf "\e[35mEnter table name:\e[0m 📛\n"
    read table_name
    if [[ -z "$table_name" ]]; then
        printf "\e[31mTable name cannot be empty.\e[0m ❌\n"
        continue
    fi

    printf "\e[32mCreating table ${table_name}\e[0m 🛠️\n"
    echo "CREATE TABLE $table_name (" > temp.txt
    echo "    ID INT AUTO_INCREMENT PRIMARY KEY," >> temp.txt

    # Loop for adding columns to the current table
    while true; do
        printf "\e[34mEnter column name (or leave empty to finish this table):\e[0m 📝\n"
        read column_name

         # Remove whitespace characters from column_name
        column_name=$(echo "$column_name" | tr -d '[:space:]')

        if [[ -z "$column_name" ]]; then
            echo "Finishing column definitions for $table_name."
            break
        fi

        source sql-column-type.sh
        source sql-column-constraints.sh

        echo "    $column_name $column_type $selected_constraints," >> temp.txt
    done

    # Remove the last comma and close the table definition
    sed -i '' -e '$ s/,$//' temp.txt
    echo ");" >> temp.txt
    echo "" >> temp.txt
    cat temp.txt >> "$SQL_FILE"
    rm temp.txt
done

printf "\e[36mGenerated SQL schema:\e[0m 🗃️\n"
cat "$SQL_FILE"
rm "$SQL_FILE.bak"
cd $PWD 
