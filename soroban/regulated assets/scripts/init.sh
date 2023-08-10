#!/bin/bash

source ./config.sh

HELP_STYLE='\e[1;32;40m' # Bold green text on a black background
STYLE='\e[1;37;44m'
NS='\033[0m' # No Color


# CONTRACT_ID=$(<"${DATA_DIR}/${COD_DEPLOY_OUTPUT_FILE}-out")
# INVOKER_SK=""

TARGET=$1
shift

if [[ "$TARGET" == "asset" ]]; then
  echo -e "\n ${STYLE}DEPLOYING REGULATED ASSET CONTRACT...${NS}"
  ./helpers/deploy.sh ${RA_WASM} ${RA_DEPLOYER_ACCOUNT_SK} ${RA_DEPLOY_OUTPUT_FILE}
  echo -e "\n ${STYLE}INITIALIZING REGULATED ASSET CONTRACT...${NS}"
  ./helpers/initialize-ra.sh
elif [[ "$TARGET" == "controller" ]]; then
  echo -e "\n ${STYLE}DEPLOYING ASSET CONTROLLER CONTRACT...${NS}"
  ./helpers/deploy.sh ${AC_WASM} ${AC_DEPLOYER_ACCOUNT_SK} ${AC_DEPLOY_OUTPUT_FILE}
else
  echo -e "\n Command invalid. Usage:"
  echo -e "  ${HELP_STYLE}$0 asset${NS} - Description for the 'asset' command."
  echo -e "  ${HELP_STYLE}$0 controller${NS} - Description for the 'controller' command."
fi
