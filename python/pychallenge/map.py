str1 = "g fmnc wms bgblr rpylqjyrc gr zw fylb. rfyrq ufyr amknsrcpq ypc dmp. bmgle gr gl zw fylb gq glcddgagclr ylb rfyr'q ufw rfgq rcvr gq qm jmle. sqgle qrpgle.kyicrpylq() gq pcamkkclbcb. lmu ynnjw ml rfc spj."

alphabet = 'abcdefghijklmnopqrstuvwxyz'
alpha_shift = 'cdefghijklmnopqrstuvwxyzab'

lst = []

for char in str1:
    indx = alphabet.find(char)
    if indx != -1:
        lst.append(alpha_shift[indx])
    else:
        lst.append(char)

new_str = ''.join(lst)
print(new_str)
