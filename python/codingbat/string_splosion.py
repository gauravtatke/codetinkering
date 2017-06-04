def string_splosion(str1):
    i = 0;
    str2 = ''
    for i in range(len(str1)):
        str2 = str2 + str1[0:i+1]
        #print (i, str2)
    return str2

print(string_splosion('code'))
