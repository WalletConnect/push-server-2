FOLDER=$1
shift  # Shifts the arguments to the left, removing the first argument (the folder)

DESCRIPTION="$1"
shift  # Shifts again to remove the second argument

while [ $# -gt 0 ]; do
    DESCRIPTION="${DESCRIPTION}-$1"
    shift
done

mkdir -p "$FOLDER"  # Creates the folder if it doesn't exist
touch "$FOLDER/$(date +%s)_$DESCRIPTION.sql"
