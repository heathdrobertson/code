#! /bin/bash
### START NOTES
### END NOTES

#### Create Text or JPG Files
makeFile(){
    # Ask user for input.
    read -p "Pick a letter t for TXT files or j for JPG files: " pick

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
    
    # Create the files.
    for i in {1..5}
    do
        touch ./assets/${file_dir}/0${i}_file.$file_ext
    done

    # Inform the user.
    echo "Creating files..."
    sleep 0.6
    ls -l ./assets/${file_dir}/
    echo ""
    echo "All $file_ext files created"
}

#### END Create Text or JPG Files

main() {
    makeFile
}
main


