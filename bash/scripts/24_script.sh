#! /bin/bash
### START NOTES
### END NOTES

source color_format.sh

current_dir=`pwd`
echo -e "${Red}Current directory: ${Yellow}${current_dir}"

file_finder(){
    echo -e ${Green}
    echo -e "Search for a term within documents in a specific directory."
    read -p "Enter a path to the directory: " dir_path
    echo -e ${Yellow}

    cd ${dir_path}

    echo -e "${Red}Files in ${Yellow}$dir_path ..."

    mkdir ./HOT-FOLDER

    for file in *; do
        if [[ -f "$file" ]]; then

            find=$(grep -ni "spo" "$file")

            if [[ -z $find ]]; then
                echo "$file is EMPTY"
            else
                echo -e "${Red}${term} FOUND in ${file}"
                echo -e "${URed}${find}${Yellow}"
                cp "$file" ./HOT-FOLDER
                echo  "****" >> HOT-FOLDER/"${file}"
            fi

        else
            echo -e "${IBBlue}$file is a FOLDER...${Yellow}"
        fi
    done
}

search_for(){
    echo $dir_path
}

main(){
    file_finder
}
main
