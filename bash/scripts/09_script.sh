#! /bin/bash
### START NOTES
### END NOTES

read -p "Type something here: " str

if [[ $str == "" ]]; then
    echo "This is an empty string."
    exit
fi

echo "- $str - moving on!"
