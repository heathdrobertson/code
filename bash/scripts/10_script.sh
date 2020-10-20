#! /bin/bash
### START NOTES
### END NOTES

read -p "Name of file or directory to check: " new_file

if [[ -e $new_file ]]; then 
    echo "$new_file exists!"
fi

if [[ ! -e $new_file ]]; then 
    echo "$new_file does not exist!"
fi



if [[ -d $new_file ]]; then 
    echo "$new_file is a directory!"
else
    echo "$new_file is not a directory!"
fi


if [[ -r $new_file ]]; then 
    echo "$new_file is readable!"
fi

if [[ -w $new_file ]]; then 
    echo "$new_file is writable!"
fi

if [[ -x $new_file ]]; then 
    echo "$new_file is executable!"
fi

if [[ -s $new_file ]]; then 
    echo "$new_file is NOT empty!"
else
    echo "$new_file is empty!"
fi

