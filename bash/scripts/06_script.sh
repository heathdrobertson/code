#! /bin/bash
### Begin Notes
## [[ ]] use for comparing strings and files (( )) use for arithmetic and comparing numbers
### End Notes
### Auto Generated Script ###

read -p "How old are you? " age

if  (( $age <= 0 )); then
    echo "You can not be 0 or negative years old!"
    exit
fi

echo "Passed negative or 0 if conditional, lets continue."
