from __future__ import annotations

class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        return longest_common_prefix3(strs)
            
                
def longest_common_prefix1(strs: List[str]) -> str:
    prefix = ''
    if not strs:
        return ''
    min_len = min([len(s) for s in strs])
    total_len = len(strs)
    for i in range(min_len):
        ch = strs[0][i]
        match = True
        for s in strs:
            if s[i] != ch:
                match = False
                break
        if match:
            prefix = prefix + ch
        else:
            break
    return prefix

def longest_common_prefix2(strs: List[str]) -> str:
    if not len(strs):
        return ''
    for i in range(len(strs[0])):
        ch = strs[0][i]
        for st in strs[1:]:
            # either index is more than string length or char does not match
            if i >= len(st) or st[i] != ch:
                # this also returns empty string if nothing matches
                return strs[0][:i]
    # when all strings are same
    return strs[0]

def longest_common_prefix3(strs: List[str]) -> str:
    if not len(strs):
        return ''
    answer = ''
    for index, letter in enumerate(strs[0]):
        for st in strs[1:]:
            if index >= len(st) or st[index] != letter:
                return answer
        answer = answer + letter
    return answer

if __name__ == '__main__':
    sol = longest_common_prefix3(['flower', 'flow', 'flight'])
    print(sol)
    