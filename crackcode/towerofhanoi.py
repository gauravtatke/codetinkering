#!/usr/bin/env python3

from listnstack import Stack

def toh(n, src, aux, dest):
    '''Move n rings from src to dest using aux'''
    if n == 2:
        aux.push(src.pop())
        dest.push(src.pop())
        dest.push(aux.pop())
    else:
        toh(n-1, src, dest, aux)
        dest.push(src.pop())
        toh(n-1, aux, src, dest)

if __name__ == '__main__':
    num = 10
    srcstack = Stack(num)
    auxstack = Stack(num)
    deststack = Stack(num)
    for i in range(10, 0, -1):
        srcstack.push(i)
    print('Source: ', srcstack)
    print('Aux: ', auxstack)
    print('Destination: ', deststack)
    toh(num, srcstack, auxstack, deststack)
    print('#############')
    print('Source: ', srcstack)
    print('Aux: ', auxstack)
    print('Destination: ', deststack)
    


