#!/bin/bash

CONTRACT_ID=$(<"${DATA_DIR}/${RA_DEPLOY_OUTPUT_FILE}-out")
INVOKER_SK=${RA_ADMIN_SK}
FUNCTION_NAME="initialize"

# Important to reload it here in case it was previously 
# deployed in the same script execution otherwise, the 
# value would've been loaded from config.sh at the very 
# beginning of the execution
AC_CONTRACT_ID_UPDT=$(<"${DATA_DIR}/${AC_DEPLOY_OUTPUT_FILE}-out")

./helpers/invoke.sh \
  ${CONTRACT_ID} \
  ${FUNCTION_NAME} \
  ${INVOKER_SK} \
   --admin ${RA_ADMIN_PK} \
   --decimal ${RA_DECIMAL} \
   --name ${RA_NAME} \
   --symbol ${RA_SYMBOL} \
   --asset_controller ${AC_CONTRACT_ID_UPDT} \
