#
#
# Syntax <FUNCTION> <ARGS>
# FUNCTION 
#     m: mint
# 
#

CONTRACT_ID=${RA_CONTRACT_ID}
INVOKER_SK=${RA_ADMIN_SK}
FUNCTION_NAME=""
ARGS=" "

FUNCTION=$1

#
# Regulated Asset functions
# =========================
case "$FUNCTION" in
  authorized) 
    FUNCTION_NAME="authorized"
    ARGS="--id $(./helpers/get-account.sh $2 pk)"
  ;;
  balance) 
    FUNCTION_NAME="balance"
    ARGS="--id $(./helpers/get-account.sh $2 pk)"
  ;;
  spendable) 
    FUNCTION_NAME="spendable_balance"
    ARGS="--id $(./helpers/get-account.sh $2 pk)"
  ;;
  allowance) 
    FUNCTION_NAME="allowance"
    ARGS="--from $(./helpers/get-account.sh $2 pk) --spender $(./helpers/get-account.sh $3 pk)"
  ;;
  decimals) 
    FUNCTION_NAME="decimals"
  ;;
  name) 
    FUNCTION_NAME="name"
  ;;
  symbol) 
    FUNCTION_NAME="symbol"
  ;;
  # quota) 
  #   if [ -z "$2"]; then
  #     FUNCTION_NAME="get_quota_time_limit"
  #   elif ["$2" = "amount"]; then
  #     FUNCTION_NAME="get_quota"
  #     ARGS="--id $(./helpers/get-account.sh $3 pk)"
  #   elif ["$2" = "release"]; then
  #     FUNCTION_NAME="get_quota_release_time"
  #     ARGS="--id $(./helpers/get-account.sh $3 pk)"
  #   fi
  # ;;
  h) 
  echo -e "       ${HELP_STYLE} name ${NS} - name."
  echo -e "       ${HELP_STYLE} symbol ${NS} - symbol."
  echo -e "       ${HELP_STYLE} decimals ${NS} - decimals."
  echo -e "       ${HELP_STYLE} authorized <USER> ${NS} - authorized."
  echo -e "       ${HELP_STYLE} balance <USER> ${NS} - balance."
  echo -e "       ${HELP_STYLE} spendable <USER> ${NS} - spendable_balance."
  echo -e "       ${HELP_STYLE} allowance <FROM> <SPENDER> ${NS} - allowance."  
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