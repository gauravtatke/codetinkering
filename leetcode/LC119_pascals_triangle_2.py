#Given a non-negative index k where k ≤ 33, return the kth index row of the Pascal's triangle.
#Note that the row index starts from 0.
#
#In Pascal's triangle, each number is the sum of the two numbers directly above it.
#
#Example:
#
#Input: 3
#Output: [1,3,3,1]

class Solution:
    def getRow(self, rowIndex: int) -> List[int]:
        if rowIndex == 0:
            return [1]
        if rowIndex == 1:
            return [1,1]
        kth_row = [1,1]
        for row in range(2, rowIndex+1):
            temp_row = []
            for col in range(row+1):
                if col == 0 or col == row:
                    temp_row.append(1)
                else:
                    temp_row.append(kth_row[col-1] + kth_row[col])
            kth_row = temp_row
        return kth_row
