#!/bin/sh

MYVAR=4

if [ "$MYVAR" -eq "GAURAV" ]; then
    echo "MY NAME"
elif [ $MYVAR = 4 ]; then
    echo "NUMBER"
else
    echo "NOT NAME"
fi
