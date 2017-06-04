def print_fibonacci(num):
    a = 0
    b = 1
    if num <= 0:
        return None
    if num == 1:
        print(a)
    elif num == 2:
        print(a,b, sep=', ', end='')
    else:
        #c = a + b
        print(a,b, sep=', ',end='')
        for i in range(3,num+1):
            c = a + b
            print(', ',,end='')
            a = b
            b = c

if __name__ == '__main__':
    num = int(input("Enter count of fibonacci number: "))
    print_fibonacci(num)

