#!/usr/bin/env python3

from collections import defaultdict

lch = 'qwertyuiopasdfghjklzxcvbnm'

d = defaultdict(lambda:None, {ord(i):i for i in lch})

print(sorted(d))
print(len(d))

mystr = '4.5.6 Document String'

print(mystr.translate(d))
