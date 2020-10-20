#! /bin/bash
### START NOTES
### END NOTES

#### Divisible by 2, 3 or 5
divisibleBy(){

    if [[ $(( $1%2 ))  == 0 ]]; 
    then
        echo "$1 divisible by 2"
    elif [[ $(( $1%3 )) == 0 ]];
    then
        echo "$1 divisible by 3"
    elif [[ $(( $1%5 )) == 0 ]];
    then
        echo "$1 divisible by 5"
    else
        echo "$1 Not divisible by 2, 3, or 5."
    fi
    
}

for i in {2,3,5,6,7,10}
do
    divisibleBy $i
done
#### END Divisible by 2, 3 or 5

