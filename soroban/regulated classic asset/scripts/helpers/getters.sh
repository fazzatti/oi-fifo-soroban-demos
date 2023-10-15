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
shift
case "$FUNCTION" in
#
# Regulated Asset functions
# =========================
  ra)
    ./helpers/get_ra.sh "$@"
  ;;
#
# Asset Controller functions
# =========================
  ac)
    ./helpers/get_ac.sh "$@"
  ;;
  h)
  echo -e "  ${SUBTITLE_STYLE} get <CONTRACT> <COMMANDS>${NS} - retrieve data from contracts."
  echo -e "  ${TITLE_STYLE}CONTRACT: ${NS}"
  echo -e "  ${SUBTITLE_STYLE} ac ${NS} - asset controller contract."
  echo -e "  ${SUBTITLE_STYLE} ra ${NS} - regulated asset contract."
  echo -e "  ${TITLE_STYLE}COMMANDS: ac ${NS}"
  ./helpers/get_ac.sh "h"
  echo -e "  ${TITLE_STYLE}COMMANDS: ra ${NS}"
  ./helpers/get_ra.sh "h"
  exit 0
  
  ;;
  *)
    echo "Error: Invalid Get function $FUNCTION" >&2 
    exit 1 
    ;;
esac


