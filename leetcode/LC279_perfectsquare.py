# Given a positive integer n, find the least number of perfect square numbers (for example, 1, 4, 9, 16, ...) which sum to n.
#
# For example, given n = 12, return 3 because 12 = 4 + 4 + 4; given n =
# 13, return 2 because 13 = 4 + 9.


def numSquares(n):
    # cntSquare[i] contains num of perfect square that add upto i
    cntSquare = [float('inf') for i in range(n + 1)]
    cntSquare[0] = 0

    for i in range(1, n + 1):
        # every number i is sum of i - j*j and j*j
        # cntSquare[i] can be cntSquare[i-j*j] + 1
        j = 1
        while j * j <= i:
            cntSquare[i] = min(cntSquare[i], cntSquare[i - j * j] + 1)
            j = j + 1
    return cntSquare[-1]


def main():
    print(numSquares(12))


if __name__ == '__main__':
    main()
