import random

def getpwd(type):
    lcase = 'abcdefghijklmnopqrstuvwxyz'
    ucase = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
    splch = '@$!_&<>'
    num = '0123456789'
    if type == 1:
        #Weak password
        lc = random.sample(lcase,random.randint(4,6))
        nu = random.sample(num,random.randint(1,3))
        passwd = lc + nu
        random.shuffle(passwd)
        passwd = ''.join(passwd)
        return passwd
    elif type == 2:
        #Strong password
        lc = random.sample(lcase,random.randint(4,6))
        uc = random.sample(ucase,random.randint(1,2))
        sp = random.sample(splch,random.randint(1,2))
        nu = random.sample(num,random.randint(1,3))
        passwd = lc + nu + uc + sp
        random.shuffle(passwd)
        passwd = ''.join(passwd)
        return passwd
    else:
        return "Incorrect choice"

def testpwd(type, rng):
    pwdlist = []
    count = 0
    for i in range(rng):
        pwd = getpwd(type)
        if pwd not in pwdlist:
            pwdlist.append(pwd)
        else:
            count = count+1
    print("Duplicate password generated in {0} are {1}".format(rng, count))

if __name__ == '__main__':
    type = int(input("Enter 1 for weak, 2 for strong : "))
    print(getpwd(type))
    #testpwd(type, 10000)
