#/usr/bin/env python3

# Given a binary array, find the maximum number of consecutive 1s in this array.

def maxConsecutiveOnes(nums):
    currlen = 0
    maxlen = 0
    for num in nums:
        if num:
            currlen += 1
        else:
            maxlen = max(maxlen, currlen)
            currlen = 0
    return max(maxlen,currlen)

def maxConsecutiveOnes2(nums):
    nums.append(0)
    indxZero = [i for i, n in enumerate(nums) if not n]
    #print(indxZero)
    #if indxZero:
    return max(n-1-indxZero[i-1]if i else n for i, n in enumerate(indxZero))
    #return len(nums) # no zeroes in list

def main():
    print(maxConsecutiveOnes2([0,0,0]))

if __name__ == '__main__':
    main()
