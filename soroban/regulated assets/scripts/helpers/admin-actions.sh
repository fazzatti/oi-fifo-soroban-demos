#
#
# Syntax <FUNCTION> <ARGS>
# FUNCTION 
#     m: mint
# 
#

get_user_pk(){
    user=$1
    case "$user" in
    a)
        echo ${USER_A_PK}
        ;;
    b)
        echo ${USER_B_PK}
        ;;
    *)
        echo "Error: Invalid user $user" >&2 # Print to stderr
        exit 1 # Exit the script
        ;;
    esac
}

get_user_sk(){
    user=$1
    case "$user" in
    a)
        echo ${USER_A_SK}
        ;;
    b)
        echo ${USER_B_SK}
        ;;
    *)
        echo "Error: Invalid user $user" >&2 # Print to stderr
        exit 1 # Exit the script
        ;;
    esac
}

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
    ARGS="--to $(get_user_pk $2) --amount $3"
    ;;
  test)
    INVOKER_SK=${AC_ADMIN_SK}
    CONTRACT_ID=${AC_CONTRACT_ID}
    FUNCTION_NAME="test"
    ARGS=""
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