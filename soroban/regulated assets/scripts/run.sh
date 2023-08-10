#!/bin/bash

source ./config.sh
export DATA_DIR="./data"



ACTOR=$1
case "$ACTOR" in
  a|b|c)
    ./helpers/user-actions.sh "$@"
    ;;
  admin)
    shift
    ./helpers/admin-actions.sh "$@"
    ;;
  *)
    echo "Error: Invalid Actor $ACTOR" >&2 # Print to stderr
    exit 1 # Exit the script
    ;;
esac


