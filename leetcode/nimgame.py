#!/usr/bin/env python3

# You are playing the following Nim Game with your friend: There is a heap of
# stones on the table, each time one of you take turns to remove 1 to 3 stones.
# The one who removes the last stone will be the winner. You will take the first
# turn to remove the stones.
#
# Both of you are very clever and have optimal strategies for the game. Write a
# function to determine whether you can win the game given the number of stones
# in the heap.
#
# For example, if there are 4 stones in the heap, then you will never win the
# game: no matter 1, 2, or 3 stones you remove, the last stone will always be
# removed by your friend.

result = False

def canIWinNimGame(n):
    # anybody who gets the number divisible by 4 will loose otherwise will win
    return n%4 != 0

def canIWinNimGame2(n):
    retval = [None for i in range(n+1)]
    return canWinGameIter(retval, n)

def canWinGame(retval, n):
    #retval[n] holds True/False for n stones
    if retval[n] is not None:
        return retval[n]
    if n in (1, 2, 3):
        # if I get 1, 2, 3 then he wins
        retval[n] = True
    elif 4 in (n-1, n-2, n-3):
        # if either ppicking 1, 2, 3, remaining becomes 4 then opposition
        # won't win so I win
        retval[n] = True
    else:
        # n-1, n-2, n-3 values will be given to opponent. if opponent wins in
        # all 3 conditions then i will always loose
        retval[n] = not (canWinGame(retval, n-1) and canWinGame(retval, n-2)
                        and canWinGame(retval, n-3))
    return retval[n]


def canWinGameIter(retval, n):
    for i in (1, 2, 3):
        retval[i] = True
    for i in range(4, n+1):
        if retval[i-1] and retval[i-2] and retval[i-3]:
            retval[i] = False
        else:
            retval[i] = True
    return retval[n]


def main():
    print('stones = ', canIWinNimGame2(1348820612))

if __name__ == '__main__':
    main()
