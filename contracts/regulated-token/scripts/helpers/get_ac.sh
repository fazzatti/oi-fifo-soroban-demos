#
#
# Syntax <FUNCTION> <ARGS>
# FUNCTION 
#     m: mint
# 
#

HELP_STYLE='\e[1;32;40m' # Bold green text on a black background
NS='\033[0m' # No Color

CONTRACT_ID=${AC_CONTRACT_ID}
INVOKER_SK=${AC_ADMIN_SK}
FUNCTION_NAME=""
ARGS=""

FUNCTION=$1

#
# Asset Controller functions
# =========================
case "$FUNCTION" in
  probation) 
    if [ -z "$2" ]; then
      FUNCTION_NAME="get_probation_period"
      ARGS=""
    else
      FUNCTION_NAME="get_account_probation_period"
      ARGS="--id $(./helpers/get-account.sh $2 pk)"
    fi
  ;;
  quota) 
    if [ -z "$2" ]; then
      FUNCTION_NAME="get_quota_time_limit"
    elif [[ "$2" == "amount" ]]; then
      FUNCTION_NAME="get_quota"
      ARGS="--id $(./helpers/get-account.sh $3 pk)"
    elif [[ "$2" == "release" ]]; then
      FUNCTION_NAME="get_quota_release_time"
      ARGS="--id $(./helpers/get-account.sh $3 pk)"
    fi
  ;;
  inflow) 
    FUNCTION_NAME="get_inflow_limit"
  ;;
  outflow) 
    FUNCTION_NAME="get_outflow_limit"
  ;;
  asset) 
    FUNCTION_NAME="get_asset"
  ;;
  admin) 
    FUNCTION_NAME="get_admin"
  ;;
  h) 
  echo -e "       ${HELP_STYLE} admin ${NS} - get_admin."
  echo -e "       ${HELP_STYLE} asset ${NS} - get_asset."
  echo -e "       ${HELP_STYLE} inflow ${NS} - get_inflow_limit."
  echo -e "       ${HELP_STYLE} outflow ${NS} - get_outflow_limit."
  echo -e "       ${HELP_STYLE} probation ${NS} - get_probation_period."
  echo -e "       ${HELP_STYLE} probation <user> ${NS} - get_account_probation_period."
  echo -e "       ${HELP_STYLE} quota ${NS} - get_quota_time_limit."
  echo -e "       ${HELP_STYLE} quota amount <user> ${NS} - get_quota."
  echo -e "       ${HELP_STYLE} quota release <user> ${NS} - get_quota_release_time."
  exit 0
  ;;
  *)
    echo "Error: Invalid Get Asset Controller function $FUNCTION" >&2 
    exit 1 
    ;;
esac



./helpers/invoke.sh \
${CONTRACT_ID} \
${FUNCTION_NAME} \
${INVOKER_SK} \
$ARGS