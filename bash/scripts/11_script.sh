#! /bin/bash
### START NOTES
### END NOTES

echo "For loops"

for i in {1..4}
do
    echo "Loop $i"
done

for i in {1.1,"dog",4,"danger","something",-98}
do
    if [[ $i == "danger" ]]; then
        echo "DANGER EXITING!"
        break
    else
        echo "Loop $i"
    fi
done


for file in ./assets/*
do
    echo "File Name: $file"
done
