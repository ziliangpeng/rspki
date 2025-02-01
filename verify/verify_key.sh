#!/bin/bash

key_name=${1:-test_key}
key_path="/tmp/.ssh/$key_name"
pub_path="$key_path.pub"

if [ ! -f "$key_path" ]; then
    echo "Key not found: $key_path"
    exit 1
fi

if [ ! -f "$pub_path" ]; then
    echo "Public key not found: $pub_path"
    exit 1
fi

ssh-keygen -y -f "$key_path" > /tmp/.ssh/generated_public_key
diff /tmp/.ssh/generated_public_key "$pub_path"
