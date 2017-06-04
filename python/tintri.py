def combineDesc(arr1, arr2):
    result = []
    idx1 = 0
    idx2 = 0
    while idx1 < len(arr1) and idx2 < len(arr2):
        if arr1[idx1] > arr2[idx2]:
            result.append(arr1[idx1]) #I can just pop the items instead of having 2 indices
            idx1 = idx1+1
        else:
            result.append(arr2[idx2])
            idx2 = idx2+1
        #idx = idx+1
    if idx1 == len(arr1) and idx2 < len(arr2):
        result.extend(arr2[idx2:])
    elif idx2 == len(arr2) and idx1 < len(arr1):
        result.extend(arr1[idx1:])
    else:
        return result
    return result


if __name__ == '__main__':
    arr1 = [10, 7, 4, 1]
    arr2 = [9, 8, 5]
    print(combineDesc(arr1, arr2))
