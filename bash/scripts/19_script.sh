#! /bin/bash
### START NOTES
### END NOTES

#### Divisible by 2, 3 or 5
divisibleBy(){

    if [[ $(( $1%2 ))  == 0 ]]; 
    then
        echo "Divisible by 2"
    elif [[ $(( $1%3 )) == 0 ]];
    then
        echo "Divisible by 3"
    else
        echo "Divisible by 5"
    fi
    
}

for i in {2,3,5}
do
    divisibleBy $i
done
#### END Divisible by 2, 3 or 5


#### Text File Counter
counter=0

txtCounter(){
    for i in ./assets/text/*.txt
    do
        (( ++counter ))
        echo "Text File #$counter"
    done
}

txtCounter
#### END Text File Counter


#### Create Text or JPG Files
makeFile(){
    read -p "Pick a letter t for TXT files or j for JPG files: " pick
    
    while [[ $pick != "j" && $pick != "t" ]];
    do
        echo  "You picked: $pick"
        read -p "Enter j for jpg or t for txt: " pick
    done

    if [[ $pick == "t" ]];
    then
        file_dir="text"
        file_ext="txt"
    else
        file_dir="images"
        file_ext="jpg"
    fi

    for i in {1..5}
    do
        touch ./assets/${file_dir}/0${i}_file.$file_ext
    done
    echo "All $file_ext files created"
}

makeFile
#### END Create Text or JPG Files

