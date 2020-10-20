#! /bin/bash
# This is a comment

mkdir assets
touch assets/1.txt assets/2.txt
ls -l >> assets/1.txt
read -p "Prefix to add to 1.txt? " pref
cp assets/1.txt assets/${pref}_1.txt
echo "Done"
