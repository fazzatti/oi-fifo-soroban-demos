#!/bin/bash

CONTRACT_ID=$(<"${DATA_DIR}/${AC_DEPLOY_OUTPUT_FILE}-out")
INVOKER_SK=${AC_ADMIN_SK}
FUNCTION_NAME="initialize"

# Important to reload it here in case it was previously 
# deployed in the same script execution otherwise, the 
# value would've been loaded from config.sh at the very 
# beginning of the execution
WI_CONTRACT_ID_UPDT=$(<"${DATA_DIR}/${WI_DEPLOY_OUTPUT_FILE}-out")

./helpers/invoke.sh \
  ${CONTRACT_ID} \
  ${FUNCTION_NAME} \
  ${INVOKER_SK} \
   --asset ${WI_CONTRACT_ID_UPDT} \
   --admin ${AC_ADMIN_PK} \
   --probation_period ${PROBATION_PERIOD} \
   --quota_time_limit ${QUOTA_TIME_LIMIT} \
   --inflow_limit ${INFLOW_LIMIT} \
   --outflow_limit ${OUTFLOW_LIMIT} \
   
  
