#import math

def __merge(numlist, lindex, mindex, rindex, reverse=False):
    n1 = mindex - lindex + 1 #no of items in left list 
    n2 = rindex - mindex #no of items in right list
    llist = []
    rlist = []
    for i in range(n1):
        llist.append(numlist[lindex+i])
    #llist.append(float('inf')) #adding infinity at last

    for j in range(n2):
        rlist.append(numlist[mindex+j+1])
    #rlist.append(float('inf'))
    i = 0
    j = 0

    if reverse: #To sort in descending order
        llist.append(float('-inf')) #adding infinity at last
        rlist.append(float('-inf'))
        for k in range(lindex, rindex+1): #loop from start to end index
            if llist[i] >= rlist[j]:
                numlist[k] = llist[i]
                i = i+1
            else:
                numlist[k] = rlist[j]
                j = j+1
    else:
        llist.append(float('inf')) #adding infinity at last
        rlist.append(float('inf'))
        for k in range(lindex, rindex+1): #loop from start to end index
            if llist[i] <= rlist[j]:
                numlist[k] = llist[i]
                i = i+1
            else:
                numlist[k] = rlist[j]
                j = j+1

def msort(numlist, beg, end, reverse=False): #merge sort
    if beg < end:
         mid = (beg + end)//2

         msort(numlist, beg, mid, reverse)
         msort(numlist, mid+1, end, reverse)
         __merge(numlist, beg, mid, end, reverse)
    return numlist


def isort(numlist, beg, end, reverse=False): #insertion sort
    i = beg #initial index
    for key in numlist[beg+1:end+1]: #from beg+1 to end
        i = i+1 #current index in numlist
        j = i-1
        if reverse:
            while j >= beg and numlist[j] < key:
                numlist[j+1] = numlist[j]
                j = j-1
            numlist[j+1] = key
        else:
            while j >= beg and numlist[j] > key:
                numlist[j+1] = numlist[j]
                j = j-1
            numlist[j+1] = key


def __partition(numlist, beg, end, reverse=False):
    key = numlist[end]
    i = beg-1
    if reverse:
        for j in range(beg, end):
            if numlist[j] >= key:
                i = i+1
                numlist[i], numlist[j] = numlist[j], numlist[i]
        numlist[i+1], numlist[end] = numlist[end], numlist[i+1]
        return i+1

    for j in range(beg, end): #should run to end-1
        if numlist[j] <= key:
            i = i+1
            numlist[i], numlist[j] = numlist[j], numlist[i] #swap the elements
    numlist[i+1], numlist[end] = key, numlist[i+1]
    return i+1


def __partition_intup(numlist, beg, end, reverse=False):
    '''
    returns a tuple (q,t) such that
    numlist[beg..q-1]< numlist[q] and numlist[t+1..end] > numlist[t]
    and numlist[q..t] are same
    '''
    q = t = end
    pivot = numlist[end]
    i = beg-1
    j = beg
    while j <= q-1:
        if numlist[j] < pivot:
            i = i+1
            numlist[i], numlist[j] = numlist[j], numlist[i]
        elif numlist[j] == pivot:
            numlist[j], numlist[q-1] = numlist[q-1], numlist[j]#copy same elem near to pivot
            q = q-1 #move q to left
            j = j-1 #run loop one more time for new value of numlist[j]
        j = j+1
    num_repeat = t-q+1
    q = i+1 #this is starting index for elements in q..t
    for k in range(num_repeat):
        #loop copies numlist[q...t] from end to correct position
        i = i+1
        numlist[i], numlist[t] = numlist[t], numlist[i]
        t = t-1 #nex iteration copies numlist[t-1]
    t = i
    #print(q,t)
    return (q, t)


def qsort(numlist, beg, end, reverse=False):
    if beg < end:
        mid = __partition(numlist, beg, end, reverse)
        qsort(numlist, beg, mid-1, reverse)
        qsort(numlist, mid+1, end, reverse)


def qsort_tup(numlist, beg, end, reverse=False):
    if beg < end:
        q, t = __partition_intup(numlist, beg, end, reverse)
        qsort_tup(numlist, beg, q-1, reverse)
        qsort_tup(numlist, t+1, end, reverse)

def bsort(numlist, beg, end, reverse=False):
    #Bubble Sort
    if reverse:
        for i in range(beg, end+1):
            for j in range(end, beg, -1):
                if numlist[j] > numlist[j-1]:
                    numlist[j], numlist[j-1] = numlist[j-1], numlist[j]
    else:
        for i in range(beg, end+1):
            for j in range(end, beg, -1):
                if numlist[j] < numlist[j-1]:
                    numlist[j], numlist[j-1] = numlist[j-1], numlist[j]

def isort_rec(numlist, beg, end, reverse=False):
    if beg == end:
        return
    key = numlist[end]
    isort_rec(numlist, beg, end-1, reverse)
    k = end-1
    if reverse:
        while k >= beg and numlist[k] < key:
            numlist[k+1] = numlist[k]
            k = k-1
        numlist[k+1] = key
    else:
        while k >= beg and numlist[k] > key:
            numlist[k+1] = numlist[k]
            k = k-1
        numlist[k+1] = key

def sort(numlist, beg=0, end=None, func=msort, reverse=False):
    if not end:
        end = len(numlist)-1
    print('Calling {0}()'.format(func))
    return func(numlist, beg, end, reverse)

if __name__ == '__main__':
    numlist1 = [3, 4, 2, 5, 9, 7, 1, 6]
    numlist2 = [23, 43, 12, 32, 65, 76, 17, 29]
    numlist3 = [5,1,7,3,4,2,5,8,4,9,5,4,10,11,17,9,5,3,4]
    sort(numlist1)
    #sort(numlist2, 0, len(numlist2)-1, func=qsort)
    #sort(numlist3, 0, len(numlist3)-1, func=bsort)
    print(numlist1)
    #print(numlist3)
