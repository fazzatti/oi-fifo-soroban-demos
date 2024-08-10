#!/bin/bash

source ./config.sh
export DATA_DIR="./data"

export MAIN_HELP='\033[1;37;44m'
export TITLE_STYLE='\e[1;33;40m'
export SUBTITLE_STYLE='\e[1;36;40m'

export HELP_STYLE='\e[1;32;40m' # Bold green text on a black background
export STYLE='\033[1;37;44m' 
export NS='\033[0m' # No Color


ACTOR=$1
case "$ACTOR" in
  a|b|c)
    ./helpers/user-actions.sh "$@"
    ;;
  admin)
    shift
    ./helpers/admin-actions.sh "$@"
    ;;
  get)
    shift
    ./helpers/getters.sh "$@"
    ;;
  h)
    echo -e "  ${MAIN_HELP}  ======= RUN ======= ${NS}"
    echo -e "  ${MAIN_HELP}  GET                  ${NS}"
    ./helpers/getters.sh "h"
    echo -e "  ${MAIN_HELP}  ======= RUN ======= ${NS}"
    echo -e "  ${MAIN_HELP}  <ACTOR> <COMMANDS>   ${NS}"
    echo -e "  ${SUBTITLE_STYLE} Actors:${NS}"
    echo -e "     admin - Contract administrator"
    echo -e "     a - user a"
    echo -e "     b - user b"
    echo -e "     c - user c"
    echo -e "  ${SUBTITLE_STYLE} admin <COMMANDS> - Admin triggered executions${NS}"
    ./helpers/admin-actions.sh "h"
    echo -e "  ${SUBTITLE_STYLE} <USER> <COMMANDS> - User triggered executions.${NS}"
    ./helpers/user-actions.sh "a" "h"

    
  ;;
  *)
    echo "Error: Invalid Actor $ACTOR" >&2 # Print to stderr
    exit 1 # Exit the script
    ;;
esac


