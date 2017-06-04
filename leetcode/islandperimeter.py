#!/usr/bin/env python3

# You are given a map in form of a two-dimensional integer grid where 1
# represents land and 0 represents water. Grid cells are connected
# horizontally/vertically (not diagonally). The grid is completely surrounded by
# water, and there is exactly one island (i.e., one or more connected land
# cells). The island doesn't have "lakes" (water inside that isn't connected to
# the water around the island). One cell is a square with side length 1. The
# grid is rectangular, width and height don't exceed 100. Determine the
# perimeter of the island.

def islandPerimeter(grid):
    perimeter = 0
    ncol = len(grid[0])
    nrow = len(grid)
    for irow, row in enumerate(grid):
        for jcol, cell in enumerate(row):
            if cell:
                if not jcol or not row[jcol-1]: # left cell
                    perimeter += 1
                if jcol+1 >= ncol or not row[jcol+1]: # right cell
                    perimeter += 1
                if irow+1 >= nrow or not grid[irow+1][jcol]:
                    perimeter += 1
                if not irow or not grid[irow-1][jcol]:
                    perimeter += 1
    return perimeter

def main():
    grid = [
            [0,1,0,0],
            [1,1,1,1],
            [0,1,0,0],
            [1,1,0,0]
            ]
    print(islandPerimeter(grid))

if __name__ == '__main__':
    main()
