#!/usr/bin/ python

sol_dict = {}
#def Memoized_maxDollar(n):
#    global sol_dict
#    return Max_dollar(n)

def Max_dollar(n):
    global sol_dict
    if n in sol_dict:
        return sol_dict[n]
    elif (n > n/2+n/3+n/4):
        sol_dict[n] = n
        return sol_dict[n]
    else:
        sol_dict[n] = Max_dollar(n/2)+Max_dollar(n/3)+Max_dollar(n/4)
        return sol_dict[n]

if __name__ == '__main__':
    n = int(raw_input("Enter any number:"))
    print "Max Dollars = ", Max_dollar(n)
