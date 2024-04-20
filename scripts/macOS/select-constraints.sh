#!/bin/bash

# Define the constraints available
constraints=("NOT NULL" "UNIQUE" "PRIMARY KEY")
printf "\e[36mAvailable constraints for the column:\e[0m 📜\n"
for i in "${!constraints[@]}"; do
    printf "\e[32m%d) %s\n\e[0m" "$((i + 1))" "${constraints[i]}"
done
printf "\e[34mEnter your choice (1-3, or press enter to finish adding constraints):\e[0m ⌨️\n"

# Function to select a single constraint
select_constraint() {
    local choice
    read choice

    # Handle the choice
    if [[ -z "$choice" ]]; then
        printf "\e[33mFinished selecting constraints.\e[0m ✅\n" >&2  # Signal completion, output to stderr
        return 1  # Exit status for finished
    elif [[ "$choice" =~ ^[1-3]$ ]]; then  # Validate input against available options
        local selected_constraint="${constraints[choice-1]}"
        printf "\e[35mYou selected: %s\e[0m\n" "$selected_constraint" >&2  # Immediate feedback, output to stderr
        echo "$selected_constraint"  # This is the only output that gets captured
        return 0  # Continue adding constraints
    else
        printf "\e[31mInvalid selection. Please select a valid option or press enter to finish.\e[0m ❌\n" >&2
        return 2  # Invalid input, output to stderr
    fi
}

# Variable to store selected constraints
selected_constraints=""

# Loop to allow adding multiple constraints
while true; do
    # Execute select_constraint function and capture its output if the selection was successful
    if constraint_output=$(select_constraint); then
        # Check if the constraint is already in the list
        if [[ ! " $selected_constraints" =~ " $constraint_output " ]]; then  # Ensuring whole words match
            selected_constraints+="$constraint_output "  # Append constraint with spaces for clear separation
        else
            printf "\e[31mConstraint '%s' is already added.\e[0m ❌\n" "$constraint_output" >&2
        fi
    else
        exit_status=$?
        if [[ $exit_status -eq 1 ]]; then  # User finished input
            break
        elif [[ $exit_status -eq 2 ]]; then  # Invalid input
            printf "\e[33mPlease try again.\e[0m 🔄\n"
        fi
    fi
done

# Trim leading and trailing spaces and replace internal spaces with a single space for final output
selected_constraints=$(echo "$selected_constraints" | sed 's/^ *//;s/ *$//;s/  */ /g')

# Output the final list of constraints
printf "\e[32mSelected constraints: $selected_constraints\e[0m 🏁\n"

