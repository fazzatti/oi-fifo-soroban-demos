

CONTRACT_ID=${RA_CONTRACT_ID}
INVOKER_SK=$(./helpers/get-account.sh $1 sk)
FUNCTION_NAME=""
ARGS=""


FUNCTION=$2
case "$FUNCTION" in
  transfer)
    FUNCTION_NAME="transfer"
    ARGS="--from $(./helpers/get-account.sh $1 pk) --to $(./helpers/get-account.sh $3 pk) --amount $4"
    ;;
   h) 
  echo -e "       ${HELP_STYLE} <USER> transfer <USER> <AMOUNT> ${NS} - User a transfer amount to user b."
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