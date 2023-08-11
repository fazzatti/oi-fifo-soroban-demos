#
#
# Syntax <ACTOR> <FUNCTION> <ARGS>
# ACTOR:
#     a: user A 
#     b: user B 
# FUNCTION 
#     t: transfer
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
    c)
        echo ${USER_C_PK}
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
    c)
        echo ${USER_C_SK}
        ;;
    *)
        echo "Error: Invalid user $user" >&2 # Print to stderr
        exit 1 # Exit the script
        ;;
    esac
}

CONTRACT_ID=${RA_CONTRACT_ID}
INVOKER_SK=$(get_user_sk $1)
FUNCTION_NAME=""
ARGS=""


FUNCTION=$2
case "$FUNCTION" in
  t)
    FUNCTION_NAME="transfer"
    ARGS="--from $(get_user_pk $1) --to $(get_user_pk $3) --amount $4"
    ;;
  bal)
    FUNCTION_NAME="balance"
    ARGS="--id $(get_user_pk $1)"
    ;;
  test)
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