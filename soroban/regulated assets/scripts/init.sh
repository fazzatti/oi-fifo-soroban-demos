#!/bin/bash

source ./config.sh

HELP_STYLE='\e[1;32;40m' # Bold green text on a black background
STYLE='\033[1;37;44m' 
NS='\033[0m' # No Color



TARGET=$1
shift
case "$TARGET" in
  all)
    echo -e "\n ${STYLE}DEPLOYING ASSET CONTROLLER CONTRACT...${NS}"
    ./helpers/deploy.sh ${AC_WASM} ${AC_DEPLOYER_ACCOUNT_SK} ${AC_DEPLOY_OUTPUT_FILE}
    
    echo -e "\n ${STYLE}DEPLOYING REGULATED ASSET CONTRACT...${NS}"
    ./helpers/deploy.sh ${RA_WASM} ${RA_DEPLOYER_ACCOUNT_SK} ${RA_DEPLOY_OUTPUT_FILE}
    
    echo -e "\n ${STYLE}INITIALIZING ASSET CONTROLLER CONTRACT...${NS}"
    ./helpers/initialize-ac.sh
    
    echo -e "\n ${STYLE}INITIALIZING REGULATED ASSET CONTRACT...${NS}"
    ./helpers/initialize-ra.sh
    ;;
  ac)
    echo -e "\n ${STYLE}DEPLOYING ASSET CONTROLLER CONTRACT...${NS}"
    ./helpers/deploy.sh ${AC_WASM} ${AC_DEPLOYER_ACCOUNT_SK} ${AC_DEPLOY_OUTPUT_FILE}
    echo -e "\n ${STYLE}INITIALIZING ASSET CONTROLLER CONTRACT...${NS}"
    ./helpers/initialize-ac.sh
    ;;
  ra)
    echo -e "\n ${STYLE}DEPLOYING REGULATED ASSET CONTRACT...${NS}"
    ./helpers/deploy.sh ${RA_WASM} ${RA_DEPLOYER_ACCOUNT_SK} ${RA_DEPLOY_OUTPUT_FILE}
    echo -e "\n ${STYLE}INITIALIZING REGULATED ASSET CONTRACT...${NS}"
    ./helpers/initialize-ra.sh
    ;;
  *)
    echo -e "\n Command invalid. Usage:"
    echo -e "  ${HELP_STYLE}$0 asset${NS} - Description for the 'asset' command."
    echo -e "  ${HELP_STYLE}$0 controller${NS} - Description for the 'controller' command."
    ;;
esac