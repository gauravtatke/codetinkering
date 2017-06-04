from datetime import date

def hundred_years(name, age, num):
    curr_year = date.today().year
    hndrd_yrs = curr_year + 100 - age
    for i in range(num):
        print("Hi {0}! You will be hundred years of age in year {1}".format(name, hndrd_yrs))

if __name__ == '__main__':
    name = input("Give me your name: ")
    while True:
        try:
            age = int(input("Give me your age: "))
            break
        except ValueError:
            print("Oops! Thas was no valid number. Try again ...\n")

    while True:
        try:
            num = int(input("How many times to print: "))
            break
        except ValueError:
             print("Oops! Thas was no valid number. Try again ...\n")

    hundred_years(name, age, num)


