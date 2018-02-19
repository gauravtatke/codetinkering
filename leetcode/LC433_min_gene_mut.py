# A gene string can be represented by an 8-character long string, with choices from "A", "C", "G", "T".
# Suppose we need to investigate about a mutation (mutation from "start" to "end"), where ONE mutation is defined as ONE single character changed in the gene string.
# For example, "AACCGGTT" -> "AACCGGTA" is 1 mutation.
# Also, there is a given gene "bank", which records all the valid gene mutations. A gene must be in the bank to make it a valid gene string.
# Now, given 3 things - start, end, bank, your task is to determine what is the minimum number of mutations needed to mutate from "start" to "end". If there is no such a mutation, return -1.
# Note:

#     Starting point is assumed to be valid, so it might not be included in the bank.
#     If multiple mutations are needed, all mutations during in the sequence must be valid.
#     You may assume start and end string is not the same.

# Example 1:

# start: "AACCGGTT"
# end:   "AACCGGTA"
# bank: ["AACCGGTA"]

# return: 1

# Example 2:

# start: "AACCGGTT"
# end:   "AAACGGTA"
# bank: ["AACCGGTA", "AACCGCTA", "AAACGGTA"]

# return: 2

# Example 3:

# start: "AAAAACCC"
# end:   "AACCCCCC"
# bank: ["AAAACCCC", "AAACCCCC", "AACCCCCC"]

# return: 3

from collections import deque


class Solution(object):
    def minMutation(self, start, end, bank):
        """
        :type start: str
        :type end: str
        :type bank: List[str]
        :rtype: int
        """

        bankset = set(bank)
        visited = set()
        dq = deque()
        dq.append(start)
        level = 0
        while dq:
            size = len(dq)
            while size:
                gene = dq.popleft()
                if gene == end:
                    return level
                gene_list = list(gene)
                for i, ch in enumerate(gene_list):
                    # oldchar = ch
                    for repch in ('A', 'C', 'G', 'T'):
                        gene_list[i] = repch
                        # print(gene_list)
                        mut_gene_str = ''.join(gene_list)
                        if mut_gene_str in bankset and mut_gene_str not in visited:
                            visited.add(mut_gene_str)
                            dq.append(mut_gene_str)
                    gene_list[i] = ch
                size -= 1
            level += 1
        return -1


def main():
    retval = Solution().minMutation("AACCGGTT", "AACCGGTA", ["AACCGGTA"])
    print(retval)


if __name__ == '__main__':
    main()