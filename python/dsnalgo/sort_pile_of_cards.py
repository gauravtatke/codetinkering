#!/usr/bin/env python3

# We have N cards with each card numbered from 1 to N. All cards are randomly shuffled. We are allowed only operation moveCard(n) which moves the card with value n to the top of the pile. You are required to find out the minimum number of moveCard() operations required to sort the cards in increasing order.

def minMove(arr):
    # start iterating from back and count number of elements already in descending order.
    # minimum movement to sort will be n-(num of elem in desc ord) because only those elem need to move
    # for e.g. 4, 2, 5, 1, 6, 3 item in desc ord are 3 i.e.{6, 5, 4} so 6 - 3 = 3 movement to sort.
    # moveCard(3) -> moveCard(2) -> moveCard(1)
    n = len(arr)
    count = 0
    for i in arr[-1::-1]:
        if i == n:
            count += 1
            n -= 1
    return len(arr)-count

def main():
    arr1 = [4, 2, 5, 1, 6, 3]
    arr2 = [5, 1, 2, 3, 4]
    arr3 = [3, 4, 2, 1]
    print(minMove(arr1))
    print(minMove(arr2))
    print(minMove(arr3))

if __name__ == '__main__':
    main()

