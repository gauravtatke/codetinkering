import Sort

def bsearch_iter(alist, key):
    beg = 0
    end = len(alist)-1
    mid = (beg+end)//2
    while beg <= end:
        if alist[mid] == key:
            return True
        elif alist[mid] > key:
            end = mid-1
        else:
            beg = mid+1

        mid = (beg+end)//2
    return False

def bsearch_rec(alist, key, beg=0, end=None):
    if end == None:
        end = len(alist)-1
    if beg > end:
        return False
    mid = (beg+end)//2
    if alist[mid] == key:
        return True
    elif alist[mid] > key:
        return bsearch_rec(alist, key, beg, mid-1)
    else:
        return bsearch_rec(alist, key, mid+1, end)

if __name__ == '__main__':
    blist = Sort.sort([23, 43, 12, 32, 65, 76, 17, 29])
    #print(bsearch_iter(blist, 30))
    print(bsearch_rec(blist, 45))
