#!/bin/bash

key_name=${1:-test_key}
key_path="/tmp/.ssh/$key_name"
pub_path="$key_path.pub"
if [ -f "$key_path" ]; then
    echo "Key already exists: $key_path. Deleting..."
    rm -f "$key_path"
fi
if [ -f "$pub_path" ]; then
    echo "Public key already exists: $pub_path. Deleting..."
    rm -f "$pub_path"
fi
mkdir -p /tmp/.ssh
echo 'y' | ssh-keygen -t rsa -b 4096 -C "test@test.com" -f "/tmp/.ssh/$key_name" -N ''
