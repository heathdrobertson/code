#! /bin/bash
### Auto Generated Script ###
# arithmetic operations or expansion uses (( )) double parentheses.

x=10
y=20

echo "SUM $((x + y))"
echo "PRODUCT $((x * y))"
echo "DIVISION $((y / x))"
echo "MODULO $((x % y))"
echo "POWER $((x**y))"
echo "--------------------------------"

# Increase value assigned to variable x before echo
((++x))
echo "Var x is now $x"
((x+=114))
echo "Var x is now $x"

echo "--------------------------------"
