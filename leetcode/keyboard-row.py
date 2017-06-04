#!/usr/bin/env python3

#Given a List of words, return the words that can be typed using letters of alphabet on only one row's of American keyboard like the image below.
#Input: ["Hello", "Alaska", "Dad", "Peace"]
#Output: ["Alaska", "Dad"]

def findWords1(words):
    klines = ['qwertyuiop','asdfghjkl','zxcvbnm']
    wlist = []
    for word in words:
        for line in klines:
            if set(word.lower()) <= set(line):
                wlist.append(word)
                break
    return wlist

def findWords2(words):
    return [word for line in ['qwertyuiop','asdfghjkl','zxcvbnm'] for word in words if set(word.lower()) <= set(line)]
    #return [word for word in words if any(set(wo[d) <= set(row) for row in ('QWERTYUIOPqwertyuiop', 'ASDFGHJKLasdfghjkl', 'ZXCVBNMzxcvbnm'))]


def main():
    findWords1(["Hello", "Alaska", "Dad", "Peace"])

if __name__ == '__main__':
    import timeit
    print(timeit.timeit('main()', setup='from __main__ import main'))
