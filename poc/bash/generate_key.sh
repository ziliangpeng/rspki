#!/bin/bash

mkdir -p /tmp/.ssh
echo 'y' | ssh-keygen -t rsa -b 4096 -C "test@test.com" -f /tmp/.ssh/test_key -N ''
