#!/usr/bin/env python3

import sys

nisland = 0

def isLoneIsland(M, i, j):
    #returns true if everything around i,j is water i.e. '0'
    #it does not check diagnal columns
    if (i-1 >= 0 and M[i-1][j] == 1) or (i+1 < len(M) and M[i+1][j] == 1) or \
       (j-1 >= 0 and M[i][j-1] == 1) or (j+1 < len(M[i]) and M[i][j+1] == 1):
        return False
    return True

def whatIs(M, i, j):
    if i < 0 or i > len(M)-1 or j < 0 or j > len(M[i])-1 or M[i][j] == 0:
        #everything outside of M is water
        return 'water'
    elif isLoneIsland(M, i, j):
        return 'island'
    else:
        return 'grpOfIsland'

def getSurroundings(M, i, j):
    #returns a dict with values populated for water, island, grpOfIsland
    surrmap = {i: 0 for i in ('water','island','grpOfIsland')}
    left = whatIs(M, i, j-1)
    right = whatIs(M, i, j+1)
    above = whatIs(M, i-1, j)
    below = whatIs(M, i+1, j)
    #x1 = y1 = x2 = y2 = -1
    for loc in (left, right, above, below):
        surrmap[loc] += 1
    if surrmap['grpOfIsland'] > 1:
        #if more than 1 grpOfIsland then we need to check if these grpOfIslands
        #are connected as one piece of land
        lcor = (i, j-1) #left co-ordinates
        rcor = (i, j+1) #right
        acor = (i-1, j) #above
        bcor = (i+1, j) #below
        visited = [[False for co in M[0]] for ro in M]
        if pathExists(M, visited, lcor, rcor):
            print('left, right')
            surrmap['grpOfIsland'] = surrmap['grpOfIsland'] - 2 + 1
        if pathExists(M, visited, lcor, acor):
            print('left, above')
            surrmap['grpOfIsland'] = surrmap['grpOfIsland'] - 2 + 1
        if pathExists(M, visited, lcor, bcor):
            print('left below')
            surrmap['grpOfIsland'] = surrmap['grpOfIsland'] - 2 + 1
        if pathExists(M, visited, rcor, acor):
            print('right above')
            surrmap['grpOfIsland'] = surrmap['grpOfIsland'] - 2 + 1
        if pathExists(M, visited, rcor, bcor):
            print('right below')
            surrmap['grpOfIsland'] = surrmap['grpOfIsland'] - 2 + 1
        if pathExists(M, visited, acor, bcor):
            print('above below')
            surrmap['grpOfIsland'] = surrmap['grpOfIsland'] - 2 + 1
    return surrmap


def staticvar(**kwargs):
    def wrapper(func):
        for key in kwargs:
            setattr(func, key, kwargs[key])
        return func
    return wrapper

@staticvar(result=False)
def pathExists(arr, visited, src, dest):
    if pathExists.result:
        return True
    x, y = src[0], src[1]
    visited[x][y] = True
    if src == dest:
        pathExists.result = True
        return True
    if not arr[x][y] or not arr[dest[0]][dest[0]]:
        #if either start or end itself is zero then return false
        return False
    if x-1 >= 0 and not visited[x-1][y] and arr[x-1][y]:
        pathExists(arr, visited, (x-1,y), dest)
    if x+1 < len(arr) and not visited[x+1][y] and arr[x+1][y]:
        pathExists(arr, visited, (x+1,y), dest)
    if y-1 >= 0 and not visited[x][y-1] and arr[x][y-1]:
        pathExists(arr, visited, (x,y-1), dest)
    if y+1 < len(arr[x]) and not visited[x][y+1] and arr[x][y+1]:
        pathExists(arr, visited, (x,y+1), dest)
    # printMatrix(visited)
    return pathExists.result


def calcNumIsland(M, i, j):
    global nisland
    if isLoneIsland(M, i, j):
        nisland += 1
        M[i][j] = 1
        return nisland
    smap = getSurroundings(M, i, j)
    if not smap['grpOfIsland']:
        #no grpOfIsland is surrounding, means surrounded by islands and water
        #all islands will make one grpOfIsland
        nisland = nisland - smap['island'] + 1
        print(smap)
    elif not smap['island']:
        #no island is surrounding, means this will be merged into grpOfIsland
        #no change in number of grpOfIsland
        #DO NOTHING
        pass
    else:
        #means it is surrounded by island and grpOfIsland
        #island & grpOfIsland will merge into one big grpOfIsland
        nisland = nisland - smap['island'] - smap['grpOfIsland'] + 1
        print(smap)
    M[i][j] = 1
    return nisland


def printMatrix(matrix):
    for row in matrix:
        for col in row:
            print(col, end='\t')
        print()

def main(argv):
    matrix = [[0 for col in range(5)] for row in range(6)]
    cord = [(0,0), (0,4), (5,0), (5,4), (1,1), (1,3), (4,1), (4,3), (2,2),
            (3,2), (0,1), (0,2), (3,4), (4,2), (1,2)]
    for i, j in cord:
        print('input = ({0},{1}), islands = '.format(i,j),
              calcNumIsland(matrix, i, j))
        printMatrix(matrix)
        print()

if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
