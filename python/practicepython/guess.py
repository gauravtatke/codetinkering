import random
import sys

def is_guess(guess, rnd):
    try:
        guess = int(guess)
    except ValueError as err:
        return -1

    if guess == rnd:
        return 0
    elif guess < rnd:
        return 1
    else:
        return 2

if __name__ == '__main__':
    num_gues = 0
    res = -1
    ans = dict([(a,b) for a,b in enumerate(['incorrect input','exactly right!', 'too low', 
        'too high'], start=-1)])
    while True:
        print("\n#### STARTING NEW GUESING GAME ###")
        rand = random.randrange(1,10)
        while True:
            guess = input("Enter your guess : ")
            if guess.lower() == "exit":
                #print("Exiting. # of guesses = ", num_gues)
                sys.exit()
            res = is_guess(guess, rand)
            num_gues = num_gues+1
            if res == 0:
                print("Guess is {0}, congrats. And you got it in {1} attempts".format(ans[res], num_gues))
                break
            print("Guess is : ", ans[res], ". Try again")
