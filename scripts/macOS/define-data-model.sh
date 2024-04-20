#!/bin/bash

# Setup and cleanup configurations
source setup.sh
echo "Writing to $SQL_FILE"
# Loop for adding tables
while true; do
    echo "Do you want to add a new table? (yes/no):"
    read answer
    if [[ "$answer" != "yes" ]]; then
        echo "Finished creating SQL schema."
        break
    fi

    echo "Enter table name:"
    read table_name
    if [[ -z "$table_name" ]]; then
        echo "Table name cannot be empty."
        continue
    fi

    echo "Creating table $table_name"
    echo "CREATE TABLE $table_name (" > temp.txt
    echo "    ID INT AUTO_INCREMENT PRIMARY KEY," >> temp.txt

    # Loop for adding columns to the current table
    while true; do
        echo "Enter column name (or leave empty to finish this table):"
        read column_name
        if [[ -z "$column_name" ]]; then
            echo "Finishing column definitions for $table_name."
            break
        fi

        source select-column-type.sh
        source select-constraints.sh

        echo "    $column_name $column_type $selected_constraints," >> temp.txt
    done

    # Remove the last comma and close the table definition
    sed -i '' -e '$ s/,$//' temp.txt
    echo ");" >> temp.txt
    cat temp.txt >> "$SQL_FILE"
    rm temp.txt
done

echo "Generated SQL schema:"
cat "$SQL_FILE"
rm "$SQL_FILE.bak"
