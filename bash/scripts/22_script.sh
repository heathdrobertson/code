#! /bin/bash
### START NOTES
### END NOTES

#### Create Text or JPG Files
renameFiles(){
    # Ask user for input.
    read -p "Rename files: enter t for TXT files or j for JPG files: " pick

    # Loop if input not "j" or "t".
    while [[ $pick != "j" && $pick != "t" ]];
    do
        echo  "You picked: $pick"
        read -p "Enter j for jpg or t for txt: " pick
    done


    # Setup file names and paths.
    if [[ $pick == "t" ]];
    then
        file_dir="text"
        file_ext="txt"
    else
        file_dir="images"
        file_ext="jpg"
    fi
    
    ls -l ./assets/${file_dir}/

    read -p "Insert a prefix to the files: " pref

    # Create the files.
    cd ./assets/${file_dir}/
    
    counter=0
    for element in *_file.$file_ext
    do
        (( ++counter ))
        if (( $counter <= 9 ));
        then
            mv ${element} ${pref}_0${counter}_file.${file_ext}
        else
            mv ${element} ${pref}_${counter}_file.${file_ext}
        fi
    done

    # Inform the user.
    cd ../../
    ls -l ./assets/${file_dir}/
    echo ""
    echo "All $file_ext files renamed"
}

#### END Create Text or JPG Files

main() {
    renameFiles
}
main

