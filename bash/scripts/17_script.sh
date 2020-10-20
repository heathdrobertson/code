#! /bin/bash
### START NOTES
### END NOTES

mydate() {
    echo "Today is: "
    date
    echo "Have a GREAT day!"
}

mydate


hello2(){
    echo "Hello $1"
    echo "Hello also to $2"
    return 44
}

hello2 "Heath" "Lisa"
echo $? # This captures the return output from the function.

