# Given a 2d grid map of '1's (land) and '0's (water), count the number of islands.
# An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically.
# You may assume all four edges of the grid are all surrounded by water.
#
# Example 1:
#
# 11110
# 11010
# 11000
# 00000
#
# Answer: 1
#
# Example 2:
#
# 11000
# 11000
# 00100
# 00011
#
# Answer: 3


def numOfIsland(grid):
    def getConnections(grid, row, col):
        # return set of connected cells including itself
        s = [(row, col)]
        if row > 0 and grid[row - 1][col]:
            s.append((row - 1, col))
        if row < len(grid) - 1 and grid[row + 1][col]:
            s.append((row + 1, col))
        if col > 0 and grid[row][col - 1]:
            s.append((row, col - 1))
        if col < len(grid[row]) - 1 and grid[row][col + 1]:
            s.append((row, col + 1))
        return frozenset(s)

    island_list = set()
    # island_list contains list of all disjoint sets
    for i, row in enumerate(grid):
        for j, col in enumerate(row):
            if col:  # ignore where col is zero
                conset = getConnections(grid, i, j)
                # print(conset)
                if len(conset) == 1:
                    # it is an island
                    island_list.add(conset)
                else:
                    to_discard = []
                    for island in island_list:
                        # if conset is any element common with island, then
                        # conset and island are connected and make one island
                        if not island.isdisjoint(conset):
                            conset = island.union(conset)
                            to_discard.append(island)
                    for s in to_discard:
                        island_list.discard(s)
                    island_list.add(conset)
    # print(island_list)
    return len(island_list)


def main():
    grid1 = [
        [1, 1, 1, 1, 0],
        [1, 1, 0, 1, 0],
        [1, 1, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ]

    grid2 = [
        [1, 1, 0, 0, 0],
        [1, 1, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 1, 1],
    ]

    grid3 = [
        [1, 0, 1, 1, 1],
        [1, 0, 1, 0, 1],
        [1, 1, 1, 0, 1]
    ]
    print(numOfIsland(grid3))


if __name__ == '__main__':
    main()
