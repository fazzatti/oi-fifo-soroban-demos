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
#
# Regulated Asset functions
# =========================
  authorized)
    INVOKER_SK=${RA_ADMIN_SK}
    CONTRACT_ID=${RA_CONTRACT_ID}
    FUNCTION_NAME="authorized"
    ARGS="--id $(./helpers/get-account.sh $2 pk)"
    ;;
  *)
    echo "Error: Invalid Get function $FUNCTION" >&2 
    exit 1 
    ;;
esac



./helpers/invoke.sh \
${CONTRACT_ID} \
${FUNCTION_NAME} \
${INVOKER_SK} \
$ARGS