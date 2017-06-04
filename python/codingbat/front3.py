def front3(st):
    if len(st) < 3:
        return st*3
    return st[:3]*3

print(front3('java'))
print(front3('bo'))
print(front3('chu'))
