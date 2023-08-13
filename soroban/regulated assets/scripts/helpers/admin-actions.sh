#
#
# Syntax <FUNCTION> <ARGS>
# FUNCTION 
#     m: mint
# 
#

INVOKER_SK=${RA_ADMIN_SK}
CONTRACT_ID=${RA_CONTRACT_ID}
FUNCTION_NAME=""
ARGS=""


FUNCTION=$1
case "$FUNCTION" in
  mint)
    FUNCTION_NAME="mint"
    ARGS="--to $(./helpers/get-account.sh $2 pk) --amount $3"
    ;;
  authorize)
    FUNCTION_NAME="set_authorized"
    ARGS="--id $(./helpers/get-account.sh $2 pk) --authorize"
    ;;
  unauthorize)
    FUNCTION_NAME="set_authorized"
    ARGS="--id $(./helpers/get-account.sh $2 pk)"
    ;;
   h) 
  echo -e "       ${HELP_STYLE} mint <USER> <AMOUNT> ${NS} - Mint x amount to user account."
  echo -e "       ${HELP_STYLE} authorize <USER>${NS} - Authorize/ Unfreeze user account."
  echo -e "       ${HELP_STYLE} unauthorize <USER>${NS} - Freeze/ Remove authorization from user account."
  exit 0
  ;;
  *)
    echo "Error: Invalid function $FUNCTION" >&2 # Print to stderr
    exit 1 # Exit the script
    ;;
esac



./helpers/invoke.sh \
${CONTRACT_ID} \
${FUNCTION_NAME} \
${INVOKER_SK} \
$ARGS