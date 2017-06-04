def find_max_subarray_brute(numlist, beg, end):
    #Brute force method to find contiguous subarray of largest sum
    max_sum = float('-inf')
    lindex = -1
    rindex = -1
    if end >= len(numlist):
        print('"end" should be less than length of list')
        raise IndexError

    for i in range(beg, end+1):
        sum_i = 0
        for j in range(i, end+1):
            sum_i = sum_i + numlist[j]
            if sum_i > max_sum:
                max_sum = sum_i
                lindex = i
                rindex = j
    return (max_sum, lindex, rindex)

def find_max_subarray_cross(numlist, beg, mid, end): #find subarray crossing the mid
    lmax_sum = float('-inf')
    lidx = -1
    lsum = 0
    for i in range(mid, beg-1, -1): #from mid down to beg
        lsum = lsum + numlist[i]
        if lsum > lmax_sum:
            lmax_sum = lsum
            lidx = i

    rsum = 0
    rmax_sum = float('-inf')
    ridx = -1
    for j in range(mid+1, end+1): #from mid+1 to end
        rsum = rsum + numlist[j]
        if rsum > rmax_sum:
            rmax_sum = rsum
            ridx = j

    return (lmax_sum+rmax_sum, lidx, ridx)

def find_max_subarray_rec(numlist, beg, end):
    if beg == end: #one element
        return (numlist[beg], beg, end)

    mid = (beg + end)//2

    lmax_sum, l_low, l_high = find_max_subarray_rec(numlist, beg, mid)
    rmax_sum, r_low, r_high = find_max_subarray_rec(numlist, mid+1, end)
    cross_max_sum, cross_low, cross_high = find_max_subarray_cross(numlist, beg, mid, end)

    if lmax_sum > rmax_sum and lmax_sum > cross_max_sum:
        return (lmax_sum, l_low, l_high)
    elif rmax_sum > lmax_sum and rmax_sum > cross_max_sum:
        return (rmax_sum, r_low, r_high)
    else:
        return (cross_max_sum, cross_low, cross_high)

def find_max_subarray_iter(numlist, beg, end): #iterative process, fastest of 3
    max_i = max_sum = numlist[beg]
    lindex = rindex = l_low = beg
    for i in range(beg+1, end+1):
        if numlist[i] < max_i + numlist[i]:
            max_i = max_i + numlist[i]
        else:
            max_i = numlist[i]
            l_low = i # saving i as left low. It may become left index depending upon next max sum
        if max_i > max_sum:
            max_sum = max_i
            lindex = l_low
            rindex = i

    return (max_sum, lindex, rindex)

def find_max_subarray_sum(numlist, beg, end): #only gives the max sum, not indices
    max_here = max_sum = numlist[beg]
    for elem in numlist[beg+1: end+1]:
        max_here = max(elem, max_here+elem)
        max_sum = max(max_here, max_sum)
    return max_sum

if __name__ == '__main__':
    numlist = [-2, 1, -3, 4, -1, 2, 1, -5, 4]
    length = len(numlist)
    max_sum, low, high = find_max_subarray_brute(numlist, 0, length-1)
    print('Max Sum = {0}, Range = {1}, {2}'.format(max_sum, low, high))

    max_sum, low, high = find_max_subarray_rec(numlist, 0, length-1)
    print('Max Sum = {0}, Range = {1}, {2}'.format(max_sum, low, high))

    max_sum, low, high = find_max_subarray_iter(numlist, 0, length-1)
    print('Max Sum = {0}, Range = {1}, {2}'.format(max_sum, low, high))

    print('Max sum = {0}'.format(find_max_subarray_sum(numlist, 1, 5)))
