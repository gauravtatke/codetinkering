def string_bits(str1):
    #i = 0
    str2 = ''
    for i in range(0,len(str1)-1,2):
        str2 = str2+str1[i]
    return str2

def string_bits2(str1):
    return str1[0::2]

def string_bits3(str1):
    lst = [str1[i] for i in range(0,len(str1)-1,2)]
    print(lst)
    return ''.join(lst)
    
print(string_bits3('gaurav tatke'))

