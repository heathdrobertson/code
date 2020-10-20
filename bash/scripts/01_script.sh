#! /bin/bash
# This is a comment


echo Hello this is my script.
read -p "What is your Name: " yourname
echo "Hello $yourname, nice to meet you."
read -p "Path to file to open in Google Chrome: " filepath
open -a "Google Chrome" ${filepath}
echo $'\n'------------------------*
