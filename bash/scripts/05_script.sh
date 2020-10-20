#! /bin/bash
### Auto Generated Script ###

echo "Hello ..."
read -p "Pick a number greater than, less than or equal to three: " guess

if (( $guess < 3 )); then
    echo "$guess < Three"
elif (( $guess == 3 )); then
    echo "$guess = Three"
else
    echo "$guess > Three"
fi
