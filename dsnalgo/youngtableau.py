def youngify_fwd(table, i, j):
    smalli = i
    smallj = j
    if i+1 < row and table[i+1][j] <= table[i][j]:
        smalli = i+1
        smallj = j
    if j+1 < col and table[i][j+1] <= table[smalli][smallj]:
        smalli = i
        smallj = j+1
    if smalli != i or smallj != j:
        table[i][j], table[smalli][smallj] = \
                table[smalli][smallj], table[i][j]
        youngify_fwd(table, smalli, smallj)

def youngify_reverse(table, i, j):
    largei = i
    largej = j
    if i-1 >= 0 and table[i-1][j] >= table[i][j]:
        largei = i-1
        largej = j
    if j-1 >= 0 and table[i][j-1] >= table[largei][largej]:
        largei = i
        largej = j-1
    if largei != i or largej != j:
        table[i][j], table[largei][largej] = \
                table[largei][largej], table[i][j]
        youngify_reverse(table, largei, largej)

def insert(table, key):
    table[row-1][col-1] = key
    youngify_reverse(table, row-1, col-1)

def extract_min(table):
    minelem = table[0][0]
    if minelem == float('inf'):
        return
    table[0][0] = float('inf')
    youngify_fwd(table, 0, 0)
    return minelem

def search_rec(table, key, i, j):
    '''
    Search starts at top right corner and moves to left and bottom
    e.g. search_rec(table, key, 0, col-1)
    '''
    if i > row-1 or j < 0:
        return False
    if table[i][j] == key:
        return True
    elif table[i][j] < key:
        return search_rec(table, key, i+1, j)
    else:
        return search_rec(table, key, i, j-1)

def search_iter(table, key):
    i = 0
    j = col-1
    while i < row and j >= 0:
        if key == table[i][j]:
            return True
        elif key > table[i][j]:
            i = i+1
        else:
            j = j-1
    return False

if __name__ == '__main__':
    aList = [5, 6, 7, 1, 10, 12, 13, 3, 4, 17, 18, 19, 2, 8, 11, 15, 21]
    row = 4
    col = 5
    tableau = [[float('inf') for j in range(col)] for i in range(row)]
    for num in aList:
        insert(tableau, num)
    print(tableau)
    print(search_rec(tableau, 28, 0, col-1))
    print(search_iter(tableau, 28))
