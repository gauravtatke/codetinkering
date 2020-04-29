def find_inversion_brute(numlist, beg, end):
    #This counts inversion and also stores indices where it occurs.
    #This is brute force and solves all problem space
    i = beg-1
    count = 0
    inver_indice = []
    for index1, elem1 in enumerate(numlist[beg:end+1], start=beg):
        i = i+1
        for index2, elem2 in enumerate(numlist[i+1:end+1], start=i+1):
            if elem1 > elem2:
                count = count+1
                inver_indice.append((index1, index2))
                #print(elem1, elem2)
    return (count, inver_indice)
    #print('Count = {0}, Inversion = {1}'.format(count, inver_indices))


def count_inversion(numlist, beg, mid, end):
    l = numlist[beg:mid+1]
    n1 = len(l)
    l.append(float('inf'))
    r = numlist[mid+1:end+1]
    r.append(float('inf'))
    i = j = 0
    count_inversion = 0
    counted = False
    for k in range(beg,end+1):
        while not counted and l[i] > r[j]:
            #To remove inversion involving inf, count inversion as soon new
            #value of r[j] is exposed
            count_inversion = count_inversion+n1-i
            counted = True
        if l[i] <= r[j]:
            #No inversion
            numlist[k] = l[i]
            i = i+1
        else:#Inversion occurs
            numlist[k] = r[j]
            j = j+1
            counted = False #set to False once new value is exposed
    return count_inversion


def find_inversion_merge(numlist, beg, end):
    #This algo only counts the inversion and is based on merge sort algo
    inv_count = 0
    if beg < end:
        mid = (beg+end)//2
        inv_count = inv_count + find_inversion_merge(numlist, beg, mid)
        inv_count = inv_count + find_inversion_merge(numlist, mid+1, end)
        inv_count = inv_count + count_inversion(numlist, beg, mid, end)
    return inv_count

if __name__ == '__main__':
    numlist = [3, 4, 2, 5, 9, 7, 1, 6]
    count, indice = find_inversion_brute(numlist, 1, 6)
    print('Count = {0}, \nInversion = {1}'.format(count, indice))
    count = find_inversion_merge(numlist, 1, 6)
    print('Count = {0}'.format(count))
