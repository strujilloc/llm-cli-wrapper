#!/bin/bash

# Find the directory of the script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
source "$SCRIPT_DIR/.env"


# Check if the API key is loaded
if [ -z "$OPENAI_API_KEY" ]; then
    echo "OPENAI API key is not set."
    exit 1
fi

if [ -z "$LLM_MODEL" ]; then
    echo "LL_model is not set."
    exit 1
fi

# Make a query to the OpenAI API
query=$1
response=$(curl -s -X POST "https://api.openai.com/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    --data '{
        "model": "'"$LLM_MODEL"'",
        "messages": [
          {
            "role": "system",
            "content": "You are a helpful assistant."
          },
          {
            "role": "user",
            "content": "'"$query"'"
          }
        ]
      }')

# Parse and output the assistant's response content
echo $response | jq -r '.choices[0].message.content'
