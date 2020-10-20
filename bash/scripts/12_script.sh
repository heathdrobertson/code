#! /bin/bash
### START NOTES
### END NOTES

echo ">>> Don't forget the spaces in comparison brackets."
a="Hello"

if [[ $a == "Hello" ]]; then
    echo "var a is 'Hello'"
fi


if (( 3 == 3 )); then
    echo "3 is equal to 3"
fi
echo -e "----------------------------------\n"

echo ">>> Pay attention to spaces in values assigned to variables."
b="Hello world!"
echo "$b"

if [[ $b == "Hello world!" ]]; then
    echo "$b It's all good."
fi

for f in * ;
do
    if [[ -x $f ]]; then
        echo "--- The file: $f is 'Executable'"
    fi
done
echo -e "----------------------------------\n"

echo ">>> Wrap variables names with \${<var_name>} to prevent issues with spaces in variable values."
c="Foo"
d="Bar"

echo "${c}${d}_Bash"
echo -e "----------------------------------\n"

echo ">>> Single quotes preserve the exact characters contained within, so variables will not be expanded."
gg="GG"
echo $gg
echo '$gg="GG"'
echo -e "----------------------------------\n"

