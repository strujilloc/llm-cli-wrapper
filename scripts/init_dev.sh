#!/bin/bash

# Define the project directory and the function name
PROJECT_DIR="$HOME/github/private/llm-cli-wrapper"
FUNCTION_NAME="aiq"

# Add the function to the .zshrc file
echo "Creating function in .zshrc..."
FUNCTION_COMMAND="
$FUNCTION_NAME() {
    PROJECT_DIR=\"$PROJECT_DIR\"
    RESPONSE=\$(cargo run --manifest-path \"\$PROJECT_DIR/Cargo.toml\" -- \"\$@\")
    echo \"\$RESPONSE\" | glow
    echo \"\$RESPONSE\" | pbcopy
}"
if grep -q "$FUNCTION_NAME()" "$HOME/.zshrc"; then
    echo "Function $FUNCTION_NAME already exists in .zshrc. Skipping."
else
    echo "$FUNCTION_COMMAND" >> "$HOME/.zshrc"
    echo "Function added to .zshrc."
fi


