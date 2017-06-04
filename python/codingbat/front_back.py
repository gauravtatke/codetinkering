def front_back(str1):
    if len(str1) == 1:
        return str1
    return str1[-1] + str1[1:-1] + str1[0]

print(front_back('code'))
print(front_back('a'))
print(front_back('ab'))
