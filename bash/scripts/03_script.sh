#! /bin/bash
# Create a new script file with a prompted name

read -p "Name of new script: " name_s
touch ${name_s}
echo "#! /bin/bash" >> ${name_s}
echo "### Auto Generated Script ###" >> ${name_s}
chmod -x ${name_s}
echo "$counter Done"
