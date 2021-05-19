class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        return str_str1(haystack, needle)

def str_str1(haystack: str, needle: str) -> int:
    start_index = 0
    i = 0
    j = 0
    if len(needle) == 0:
        return 0
    if len(needle) > len(haystack):
        return -1
    while True:
        i = 0
        while j < len(haystack) and needle[i] != haystack[j]:
            j = j+1
        if j >= len(haystack) or len(needle) > len(haystack) - j:
            # no matching charactor found
            return -1
        start_index = j
        while i < len(needle) and j < len(haystack) and len(needle[i:]) <= len(haystack) - j and needle[i] == haystack[j]:
            i = i+1
            j = j+1
        if i >= len(needle):
            # complete match found
            return start_index
        else:
            # start the loop again with j pointing next char of first match index
            j = start_index + 1
    return -1

if __name__ == '__main__':
    needle = "issipi"
    haystack = "mississippi"
    index = Solution().strStr(haystack, needle)
    print(f'needle {needle}, haystack {haystack}, index {index}')