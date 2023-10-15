

# 0: PK
# 1: SK
USER_A=("$USER_A_PK" "$USER_A_SK")
USER_B=("$USER_B_PK" "$USER_B_SK")
USER_C=("$USER_C_PK" "$USER_C_SK")


user=$1
key=$2
i=""

case "$key" in
    pk)
        i=0
        ;;
    sk)
        i=1
        ;;
    *)
        echo "Error: Invalid key requested $key" >&2 
        exit 1 
        ;;
esac



case "$user" in
    a)
        echo "${USER_A[$i]}"
        ;;
    b)
        echo "${USER_B[$i]}"
        ;;
    c)
        echo "${USER_C[$i]}"
        ;;
    *)
        echo "Error: Invalid user $user" >&2 
        exit 1 
        ;;
esac
