#! /bin/bash
### START NOTES
### END NOTES


source color_format.sh

HOTFOLDER="./assets/text/HOT-FOLDER" 
mkdir $HOTFOLDER

get_file(){
    local str="$1"
    local fil="$2"
    
    check=$(grep -ni "$str" "$fil")

    local name_file=$(basename "$fil")
    if [[ -z "$check" ]]; then
        echo "- $name_file"
    else
        echo -e "${Green}- $name_file:${White}"

        cp "$fil" $HOTFOLDER/"$name_file"
        echo " " >> $HOTFOLDER/"$name_file"
        echo "*-------##****" >> $HOTFOLDER/"$name_file"
        echo "$check" >> $HOTFOLDER/"$name_file"
    fi
}


rm -r $HOTFOLDER
mkdir $HOTFOLDER

for i in $(find ./assets -type d); do

    if [[ "$i" != $HOTFOLDER ]]; then
        echo -e "${Red}Directory: $i ${White}"
        for myfile in $i/*; do
            if [[ -f "$myfile" ]]; then
                get_file "spell" "$myfile"
            fi
        done
    fi
    echo " "
done
