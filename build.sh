#! /bin/bash

# exit if command fails
set -e

cross build --target arm-unknown-linux-gnueabi

scp target/arm-unknown-linux-gnueabi/debug/control_api drake@192.168.50.105:~
