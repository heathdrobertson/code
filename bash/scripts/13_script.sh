#! /bin/bash
### START NOTES
### END NOTES

echo "Start while loops."

num=0

while (( $num <= 10 ));
do
    echo "Num is: $num"
    (( ++num ))
done

fib=0
fib2=1

while (( $fib2 <= 1000 ));
do
    echo "Fib num is: $fib"
    nth=$(( fib + fib2 ))
    fib=$fib2
    fib2=$nth
done
