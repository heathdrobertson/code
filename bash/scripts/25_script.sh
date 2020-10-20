#! /bin/bash
### START NOTES
### END NOTES

source color_format.sh

HOTFOLDER="./assets/text/HOT-FOLDER" 
mkdir $HOTFOLDER

f=$(find ./assets -type d)

for i in $f; do

    if [[ "$i" != $HOTFOLDER ]]; then
        echo -e "${White}This is the folder: $i ${Blue}"
        for file in $i/* ; do
            if [[ -f "$file" ]]; then
                echo -e "${Yellow}The file inside is: $file"

                find=$(grep -ni "spo" "$file")
                if [[ -n $find ]]; then
                    echo -e "${Red}${term}FOUND in ${file}"
                    echo -e "${URed}${find}${Yellow}"
                    file_name=$(basename $file)
                    cp "$file" $HOTFOLDER 
                    echo  "****" >> $HOTFOLDER/"${file_name}"
                fi
            fi
        done
        echo -e "${Red}------------------------------------------"
        echo " "
    fi

done
