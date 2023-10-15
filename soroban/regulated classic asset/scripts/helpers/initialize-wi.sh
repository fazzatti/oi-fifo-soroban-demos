#!/bin/bash

CONTRACT_ID=$(<"${DATA_DIR}/${WI_DEPLOY_OUTPUT_FILE}-out")
INVOKER_SK=${WI_ADMIN_SK}
FUNCTION_NAME="initialize"

# Important to reload it here in case it was previously 
# deployed in the same script execution otherwise, the 
# value would've been loaded from config.sh at the very 
# beginning of the execution
CA_CONTRACT_ID_UPDT=$(<"${DATA_DIR}/${CA_DEPLOY_OUTPUT_FILE}-out")

./helpers/invoke.sh \
  ${CONTRACT_ID} \
  ${FUNCTION_NAME} \
  ${INVOKER_SK} \
   --asset ${CA_CONTRACT_ID_UPDT} \
   --admin ${WI_ADMIN_PK} \
   --asset_controller ${CA_CONTRACT_ID_UPDT} \
   
  
