# Environment Variables
export NETWORK_NAME="testnet" # Name of the Stellar network to deploy the contract (e.g., public, testnet, futurenet)
export DATA_DIR="./data"

#Accounts
export ADMIN_PK="GALIALRZJ5EU2IJJSIQEA3D3ZIEHK5HPBHZJFUEPTGQU3MYEKKIUINTY"  #The Public key of the Admin account
export ADMIN_SK="SCVCU3KWSSLML7MVWKJJB5VKK6R3CMILO5733IBJJ3C5APBWJUOCDUVY"  #The Secret key of the Admin account

# Asset Controller
export AC_WASM="../../../target/wasm32-unknown-unknown/release/asset_controller.wasm"
export AC_DEPLOYER_ACCOUNT_SK=${ADMIN_SK}
export AC_DEPLOY_OUTPUT_FILE="ac-deploy"
export AC_CONTRACT_ID=$(<"${DATA_DIR}/${AC_DEPLOY_OUTPUT_FILE}-out")

export AC_ADMIN_PK=${ADMIN_PK}
export AC_ADMIN_SK=${ADMIN_SK}

export OUTFLOW_LIMIT=100000000000 #10000
export INFLOW_LIMIT=70000000000   #7000
export QUOTA_TIME_LIMIT=600  #10 minutes
export PROBATION_PERIOD=5184000 #1 day


# Regulated Asset
export RA_WASM="../../../target/wasm32-unknown-unknown/release/regulated_asset.wasm"
export RA_DEPLOYER_ACCOUNT_SK=${ADMIN_SK}
export RA_DEPLOY_OUTPUT_FILE="ra-deploy"
export RA_CONTRACT_ID=$(<"${DATA_DIR}/${RA_DEPLOY_OUTPUT_FILE}-out")


export RA_ADMIN_PK=${ADMIN_PK}
export RA_ADMIN_SK=${ADMIN_SK}
export RA_DECIMAL=7
export RA_NAME="FifoCoin"
export RA_SYMBOL="FIFO"


#USER ACCOUNTS
export USER_A_PK="GC45QSBFYHGQUIWWQEOZ43INQGXX57CSSAABWRZ325H7MNFIFWZ56FD4"   #The Public key of the account that represents a user
export USER_A_SK="SCMEJMTU57FOOUW5JOKIEGUHLORE36JJKHMLS2E6QPGDQ32CRWFWIZGD"   #The Secret key of the account that represents a user
export USER_B_PK="GDOGCPASYE37XXAEDCY5DKDXUFBLJC63NWQFPQWUP6AA6DQCSULUOUK5"   #The Public key of the account that represents a user
export USER_B_SK="SA5FLB42EENZQWTYJASFY4N43J257BKY5ZET6VIVU625KONF5P7P6XEP"   #The Secret key of the account that represents a user
export USER_C_PK="GBWMCTQCRROOLYVBXQGJLXRXNPKAN2GFGNB7RFJOSVLD7EBQF4OI2GDM"   #The Public key of the account that represents a user
export USER_C_SK="SCOMBFLTEGYE7JRIGVJHYWU5AAT7H2IRSJACWPY5BAQV5K7W7UTKR4Z3"   #The Secret key of the account that represents a user

