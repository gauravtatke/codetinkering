# Initially, there is a Robot at position (0, 0). Given a sequence of its moves, judge if this robot makes a circle, which means it moves back to the original place.
# The move sequence is represented by a string. And each move is represent by a character. The valid robot moves are R (Right), L (Left), U (Up) and D (down). The output should be true or false representing whether the robot makes a circle.
#
# Example 1:
# Input: "UD"
# Output: true
#
# Example 2:
# Input: "LL"
# Output: false

from collections import Counter


def judgeCircle(moves):
    starti = startj = 0
    for move in moves:
        if move == 'L':
            startj = startj - 1
        if move == 'R':
            startj = startj + 1
        if move == 'U':
            starti = starti - 1
        if move == 'D':
            starti = starti + 1

    return starti == 0 and startj == 0


def judgeCircle_2(moves):
    return moves.count('L') == moves.count('R') and moves.count('U') == moves.count('D')


def judgeCircle_3(moves):
    c = Counter(moves)
    return c['L'] == c['R'] and c['U'] == c['D']


def main():
    print(judgeCircle('UD'))
    print(judgeCircle('LL'))

    print(judgeCircle_2('UD'))
    print(judgeCircle_2('LL'))

    print(judgeCircle_3('UD'))
    print(judgeCircle_3('LL'))


if __name__ == '__main__':
    main()
