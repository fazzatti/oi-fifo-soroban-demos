#!/bin/bash

source ./config.sh

HELP_STYLE='\e[1;32;40m' # Bold green text on a black background
STYLE='\033[1;37;44m' 
NS='\033[0m' # No Color



TARGET=$1
shift
case "$TARGET" in
  all)

    echo -e "\n ${STYLE}WRAPPING ASSET...${NS}"
    ./helpers/asset-wrap.sh

    echo -e "\n ${STYLE}DEPLOYING WRAPPER INTERFACE CONTRACT...${NS}"
    ./helpers/deploy.sh ${WI_WASM} ${WI_DEPLOYER_ACCOUNT_SK} ${WI_DEPLOY_OUTPUT_FILE}
    
    echo -e "\n ${STYLE}DEPLOYING ASSET CONTROLLER CONTRACT...${NS}"
    ./helpers/deploy.sh ${AC_WASM} ${AC_DEPLOYER_ACCOUNT_SK} ${AC_DEPLOY_OUTPUT_FILE}
    
    echo -e "\n ${STYLE}INITIALIZING ASSET CONTROLLER CONTRACT...${NS}"
    ./helpers/initialize-ac.sh

    echo -e "\n ${STYLE}INITIALIZING WRAPPER INTERFACE CONTRACT...${NS}"
    ./helpers/initialize-wi.sh

    ;;
  ac)
    echo -e "\n ${STYLE}DEPLOYING ASSET CONTROLLER CONTRACT...${NS}"
    ./helpers/deploy.sh ${AC_WASM} ${AC_DEPLOYER_ACCOUNT_SK} ${AC_DEPLOY_OUTPUT_FILE}
    echo -e "\n ${STYLE}INITIALIZING ASSET CONTROLLER CONTRACT...${NS}"
    ./helpers/initialize-ac.sh
    ;;
  wi)
    echo -e "\n ${STYLE}DEPLOYING WRAPPER INTERFACE CONTRACT...${NS}"
    ./helpers/deploy.sh ${WI_WASM} ${WI_DEPLOYER_ACCOUNT_SK} ${WI_DEPLOY_OUTPUT_FILE}
    echo -e "\n ${STYLE}INITIALIZING WRAPPER INTERFACE CONTRACT...${NS}"
    ./helpers/initialize-wi.sh
    ;;
  *)
    echo -e "\n Usage: init.sh <COMMAND>"
    echo -e "  ${HELP_STYLE}$0 all${NS} - Deploys and initialize the whole use case."
    echo -e "  ${HELP_STYLE}$0 wi${NS} - Deploys and initialize the wrapper interface."
    echo -e "  ${HELP_STYLE}$0 ac${NS} - Deploys and initialize the asset controller."
    ;;
esac