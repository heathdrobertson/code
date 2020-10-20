#! /bin/bash
### START NOTES
### END NOTES


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

