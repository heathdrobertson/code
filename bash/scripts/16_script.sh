#! /bin/bash
### START NOTES
# Exit States $?
### END NOTES


# Sample shell script to demo exit code usage #
#
 

## find ip in the file ##
grep -q 192.168.2.254 ./assets/config.yaml
 
## Did we found IP address? Use exit status of the grep command ##
if [[ $? == 0 ]];
then
  echo "Success: I found IP address in file."
  exit 0
elif [[ $? == 1 ]];
then
  echo "Failure: I did not find IP address in file. Script failed"
  exit 1
else
  echo "Error: there was an error."
  exit 2
fi

echo $?
