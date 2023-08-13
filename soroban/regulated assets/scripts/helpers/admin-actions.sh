#
#
# Syntax <FUNCTION> <ARGS>
# FUNCTION 
#     m: mint
# 
#

CONTRACT_ID=""
INVOKER_SK=""
FUNCTION_NAME=""
ARGS=""


FUNCTION=$1
case "$FUNCTION" in
  m|M)
    INVOKER_SK=${RA_ADMIN_SK}
    CONTRACT_ID=${RA_CONTRACT_ID}
    FUNCTION_NAME="mint"
    ARGS="--to $(./helpers/get-account.sh $2 pk) --amount $3"
    ;;
  quota)
    INVOKER_SK=${AC_ADMIN_SK}
    CONTRACT_ID=${AC_CONTRACT_ID}
    FUNCTION_NAME="get_quota"
    ARGS="--id $(./helpers/get-account.sh $2 pk)"
    ;;
  authorize)
    INVOKER_SK=${RA_ADMIN_SK}
    CONTRACT_ID=${RA_CONTRACT_ID}
    FUNCTION_NAME="set_authorized"
    ARGS="--id $(./helpers/get-account.sh $2 pk) --authorize"
    ;;
  unauthorize)
    INVOKER_SK=${RA_ADMIN_SK}
    CONTRACT_ID=${RA_CONTRACT_ID}
    FUNCTION_NAME="set_authorized"
    ARGS="--id $(./helpers/get-account.sh $2 pk)"
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