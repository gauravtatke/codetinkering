def ispalindrome(str1):
    for indx in range(len(str1)//2):
        if str1[indx] != str1[-(indx+1)]:
            #print(str1[indx+1], str1[-(indx+1)])
            return False
    return True

if __name__ == '__main__':
    str2 = input("Enter string: ")
    print(ispalindrome(str2))

