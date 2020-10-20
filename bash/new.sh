#! /bin/bash

counter=0

for entry in ./scripts/*
do
    if [[ $entry =~ $_script.sh ]]; then
        ((++counter))
        echo "$counter Script"
    else
        echo "$entry"
    fi
done

((++counter))
if (( $counter < 10 )); then
    script_name=scripts/0${counter}_script.sh
else
    script_name=scripts/${counter}_script.sh
fi

touch $script_name
echo "#! /bin/bash" >> ${script_name}
echo "### START NOTES" >> ${script_name}
echo "### END NOTES" >> ${script_name}
chmod u=rwx ${script_name}
echo "$counter Script Created - Done"

read -p "Enter new script description: " description
echo "    - "\[${counter}_script.sh\]\(/$script_name\)" - $description" >> assets/CONTENT.md
cat assets/CONTENT.md > README.md
cat assets/FOOTER.md >> README.md
