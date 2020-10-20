#! /bin/bash
### START NOTES
### END NOTES

echo "Hello"

if [[ "abcd" == "abcd" ]]; then
    echo "Equal...!"
fi

echo "Moving on..."

if [[ "abcD" != "abcd" ]]; then
    echo "Strings are different...!"
fi

echo "End"

read -p "Type the password to know the file name: " pass
if [ $pass == "Table1X" ]; then
 	echo "Correct!  the file name is:  catfile.txt"
else
	echo "incorrect"
	exit
fi
