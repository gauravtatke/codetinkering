def traverse(mat, row, col):
    global result
    if result:
        return result
    arrive[row][col] = True
    if row == nRow-1 and col == nCol-1:
        result = True
        return result
    if not mat[row][col]: # starting point itself is zero
        return result
    #move right, down, left, up
    if col+1 <= nCol-1 and not arrive[row][col+1] and mat[row][col+1]:
        traverse(mat, row, col+1)
    if row+1 <= nRow-1 and not arrive[row+1][col] and mat[row+1][col]:
        traverse(mat, row+1, col)
    if col-1 >= 0 and not arrive[row][col-1] and mat[row][col-1]:
        traverse(mat, row, col-1)
    if row-1 >= 0 and not arrive[row-1][col] and mat[row-1][col]:
        traverse(mat, row-1, col)
    return result

result = False

def prettyprint(alist):
    for row in alist:
        print(row, end='\n')

if __name__ == '__main__':
    nRow, nCol = 6, 6
    arrive = [[False for co in range(nCol)] for ro in range(nRow)]
    #prettyprint(arrive)
    mat1 = [
        [1,1,1,0,0,0],
        [0,0,1,1,0,0],
        [1,0,0,1,0,0],
        [1,0,1,1,0,0],
        [0,1,1,0,0,0],
        [0,0,1,1,1,1]
        ]

    mat2 = [
        [1,0,1,1,1,0],
        [1,1,1,0,1,0],
        [0,0,1,1,1,0],
        [1,1,1,1,0,0],
        [0,0,0,0,0,1],
        [0,0,0,0,1,1]
        ]

    print(traverse(mat2, 0, 0))
    prettyprint(arrive)
