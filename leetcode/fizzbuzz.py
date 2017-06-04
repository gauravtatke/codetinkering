#!/usr/bin/env python3

# Write a program that outputs the string representation of numbers from 1 to n.
#
# But for multiples of three it should output “Fizz” instead of the number and
# for the multiples of five output “Buzz”. For numbers which are multiples of
# both three and five output “FizzBuzz”.
#
# n = 15,
# Return:
# ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14",
#  "FizzBuzz"]

def fizzbuzz(n):
    res = []
    for num in range(1, n+1):
        if not num%15:
            res.append('FizzBuzz')
        elif not num%3:
            res.append('Fizz')
        elif not num%5:
            res.append('Buzz')
        else:
            res.append(str(num))
    return res

def fizzbuzz2(n):
    res = []
    num = 0
    fizz = buzz = 0
    while num < n:
        fizz += 1
        buzz += 1
        num += 1
        if fizz == 3 and buzz == 5:
            res.append('FizzBuzz')
            fizz = 0
            buzz = 0
        elif fizz == 3:
            res.append('Fizz')
            fizz = 0
        elif buzz == 5:
            res.append('Buzz')
            buzz = 0
        else:
            res.append(str(num))
    return res


def fizzbuzz3(n):
    # 'str' * 0 or 'str' * False = ''
    # 'str' * 1 or 'str' * True = 'str'
    res = ['Fizz'*(not num%3) + 'Buzz'*(not num%5) or str(num) for num in \
            range(1, n+1)]
    return res

def main():
    print(fizzbuzz(15))
    print(fizzbuzz2(15))
    print(fizzbuzz3(15))

if __name__ == '__main__':
    main()
