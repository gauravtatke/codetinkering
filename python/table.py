#! /usr/etc/ python

number = int(raw_input("Enter any number : "))

for i in range(1,11):
    print '%d X %d = %d' %(number,i,number*i)
