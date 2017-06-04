#!/usr/bin/env python3

# Given a word, you need to judge whether the usage of capitals in it is right
# or not.
#
# We define the usage of capitals in a word to be right when one of the
# following cases holds:
#
# All letters in this word are capitals, like "USA".
# All letters in this word are not capitals, like "leetcode".
# Only the first letter in this word is capital if it has more than one letter,
# like "Google".
# Otherwise, we define that this word doesn't use capitals in a right way.
#
# Example 1:
# Input: "USA"
# Output: True
# Example 2:
# Input: "FlaG"
# Output: False

def detectCapitalUse(word):
    if word.isupper():
        return True
    elif word.islower():
        return True
    elif word[0].islower():
        return False
    elif word[0].isupper() and word[1:].islower():
        return True
    else:
        return False

def detectCapitalUse2(word):
    return word.islower() or word.isupper() or word.istitle()

def detectCapitalUse3(word):
    count = 0
    for ch in word:
        if ch.isupper():
            count += 1
    return (count == 0 or count == len(word)) or\
            (count == 1 and word[0].isupper())

def main():
    print("leet = ", detectCapitalUse3("leet"))
    print('Leet = ', detectCapitalUse3('Leet'))
    print('leEt', detectCapitalUse3('leEt'))
    print('LeEt', detectCapitalUse3('leEt'))
    print('LEET', detectCapitalUse3('LEET'))

if __name__ == '__main__':
    main()
