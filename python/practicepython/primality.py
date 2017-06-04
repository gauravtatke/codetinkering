import math
import sys

num = int(input("Enter number : "))

if num % 2 == 0 or num % 3 == 0:
    print("Not pime")
    sys.exit()

for i in range(5,int(math.sqrt(num)+1)):
    if num % i == 0:
        print("not prime")
        sys.exit()

print("prime")


