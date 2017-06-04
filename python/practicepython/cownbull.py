import random
import sys

def getdigits(num):
    dig = []
    while num:
        n = num % 10
        num = num // 10
        dig.append(n)
    return dig

def cowbull(rlist,ulist):
    #print(rlist, ulist, sep=', ')
    cw,bl = 0,0
    for index in range(4):
        if ulist[index] == rlist[index]:
            cw = cw + 1
        elif ulist[index] in rlist:
            bl = bl + 1
    return (cw,bl)

if __name__ == '__main__':
    rnd_num = random.randint(1000,9999)
    randlist = getdigits(rnd_num)

    while True:
        try:
            usr_num = int(input('Enter Number : '))
        except ValueError as err:
            print("Oops! No strings, only 4 digit numbers",err)
        else:
            if usr_num < 1000 or usr_num > 9999:
                print('Only 4 digit number')
                continue
            elif usr_num == rnd_num:
                print("Correct answer!")
                break
            else:
                #randlist = getdigits(rnd_num)
                usrlist = getdigits(usr_num)
                #print(randlist, usrlist, sep=', ')
                cwbl = cowbull(randlist, usrlist)
                print("{0} cow, {1} bull...Try again".format(*cwbl))


