#! /bin/bash
### START NOTES
### END NOTES

myfunction(){
    local var1="Eggs"
    var2="Salad"

    echo "\$var1 is a local variable and inside the function is: $var1"
    echo "\$var2 is a global variable and inside the function is: $var2"
}

myfunction

echo ">>>"
echo ">>> A variable declared as local is one that is visible only within the block of code in which it appears. It has local scope. In a function, a local variable has meaning only within that function block."
echo ">>>"

echo "Outside \$var1 is declared as a local variable and is: $var1"
echo "Outside \$var2 is: $var2"

