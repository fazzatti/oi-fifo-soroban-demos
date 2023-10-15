# GENERAL
# ---------------------------------------------------------------------------
#
export NETWORK_NAME="testnet" # Name of the Stellar network to deploy the contract (e.g., public, testnet, futurenet)
export DATA_DIR="./data"
export DEFAULT_DECIMAL=7
export FEE=$((10 * 10**DEFAULT_DECIMAL)) # Base fee in XLM 

export ADMIN_PK="GCBEV5EHO7BKDKQNENN6MD46SJ6OTWZX3SRJXXDXBM4T6PMIQ63SFZJC"  #The Public key of the Admin account. Must be the asset Issuer.
export ADMIN_SK="SDIAH2RGIOLOWCSLJ76OCHWB5M6BTAI4OVTCHKK4FDL4WDZ7DDT2MEQQ"  #The Secret key of the Admin account


# CLASSIC ASSET
# ---------------------------------------------------------------------------
# Important: the Issuer account must have its authorization flags set 
# beforehand.  
#
export CA_ISSUER_PK=${ADMIN_PK}
export CA_ISSUER_SK=${ADMIN_SK}
export CA_DECIMAL=7
export CA_NAME="FifoCoin"
export CA_SYMBOL="FIFO4"
export CA_ID="${CA_SYMBOL}:${CA_ISSUER_PK}" 
export CA_DEPLOY_OUTPUT_FILE="ca-deploy"
export CA_CONTRACT_ID=$(<"${DATA_DIR}/${CA_DEPLOY_OUTPUT_FILE}-out")

# WRAPPER INTERFACE
# ---------------------------------------------------------------------------
#
export WI_WASM="../../../target/wasm32-unknown-unknown/release/wrapper_interface.wasm"
export WI_DEPLOYER_ACCOUNT_SK=${ADMIN_SK}
export WI_DEPLOY_OUTPUT_FILE="wi-deploy"
export WI_CONTRACT_ID=$(<"${DATA_DIR}/${WI_DEPLOY_OUTPUT_FILE}-out")
export WI_ADMIN_PK=${ADMIN_PK}
export WI_ADMIN_SK=${ADMIN_SK}

# ASSET CONTROLLER
# ---------------------------------------------------------------------------
#
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


# USER ACCOUNTS
# ---------------------------------------------------------------------------
# Important: All user accounts must have the trustline created beforehand!
# Also make sure to set the asset profile before creating the trustlines so
# effects like AUTH_REQUIRED are enforced in the trustline during creation.
#
export USER_A_PK="GCXATMQXBLTUH2RILL7CRHATQOKUUAFTGIFQ5GF7P64YOUEFML3G5SSC"   #The Public key of the account that represents a user
export USER_A_SK="SDFV7SM3RIVI7C46UXET5AR3AE3ADLXA6E72DF7B4ETLRHYAMSTOWD2Y"   #The Secret key of the account that represents a user
export USER_B_PK="GAYW6NEHFWTHFKHHJIPHUJONUUXB5UHXJCSRP2XKUI2F3JLUF2X4QKRI"   #The Public key of the account that represents a user
export USER_B_SK="SD4RJU4FB6CLVKRNVB4Z4R4Q2QN55AK3WKTM3DCEZVYX4UZCXFPUMOSW"   #The Secret key of the account that represents a user
# export USER_C_PK="GBWMCTQCRROOLYVBXQGJLXRXNPKAN2GFGNB7RFJOSVLD7EBQF4OI2GDM"   #The Public key of the account that represents a user
# export USER_C_SK="SCOMBFLTEGYE7JRIGVJHYWU5AAT7H2IRSJACWPY5BAQV5K7W7UTKR4Z3"   #The Secret key of the account that represents a user




# Asset Controller
# export AC_WASM="../../../target/wasm32-unknown-unknown/release/asset_controller.wasm"
# export AC_DEPLOYER_ACCOUNT_SK=${ADMIN_SK}
# export AC_DEPLOY_OUTPUT_FILE="ac-deploy"
# export AC_CONTRACT_ID=$(<"${DATA_DIR}/${AC_DEPLOY_OUTPUT_FILE}-out")

# export AC_ADMIN_PK=${ADMIN_PK}
# export AC_ADMIN_SK=${ADMIN_SK}

# export OUTFLOW_LIMIT=100000000000 #10000
# export INFLOW_LIMIT=70000000000   #7000
# export QUOTA_TIME_LIMIT=600  #10 minutes
# export PROBATION_PERIOD=5184000 #1 day


# # Regulated Asset
# export RA_WASM="../../../target/wasm32-unknown-unknown/release/regulated_asset.wasm"
# export RA_DEPLOYER_ACCOUNT_SK=${ADMIN_SK}
# export RA_DEPLOY_OUTPUT_FILE="ra-deploy"
# export RA_CONTRACT_ID=$(<"${DATA_DIR}/${RA_DEPLOY_OUTPUT_FILE}-out")


# export RA_ADMIN_PK=${ADMIN_PK}
# export RA_ADMIN_SK=${ADMIN_SK}
# export RA_DECIMAL=7
# export RA_NAME="FifoCoin"
# export RA_SYMBOL="FIFO"


