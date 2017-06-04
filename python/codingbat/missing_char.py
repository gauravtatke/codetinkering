def missing_char(str1, n):
    return str1[:n] + str1[n+1:]

print(missing_char('kitten', 1))
print(missing_char('kitten', 0))
print(missing_char('kitten', 4))
