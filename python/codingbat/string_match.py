def string_match(str1, str2):
    minlen = min(len(str1), len(str2))
    count = 0
    for i in range(minlen-1):
        if str1[i:i+2] == str2[i:i+2]:
            count = count+1
    return count

print(string_match('abc', 'acckgabc'))
