#! /bin/bash
### START NOTES
### END NOTES

read -p "How old are you? " age

if (( $age < 0 || $age > 110 )); then
    echo "Number is not within a normal human lifespan."
    exit
fi

if (( $age < 64 && $age > 26 )); then
    echo "You are between 26 and 64 years old"
    exit
fi


echo "Lets continue"
