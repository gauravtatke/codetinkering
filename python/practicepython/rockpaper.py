def winner(player1, player2):
    if player1 == player2:
        return -1

    if player1 == 0:
        if player2 == 1:
            return "player2"
        else:
            return "player1"

    if player1 == 1:
        if player2 == 0:
            return "player1"
        else:
            return "player2"

    if player1 == 2:
        if player2 == 0:
            return "player2"
        else:
            return "player1"

if __name__ == '__main__':
    options = {opt:num for num,opt in enumerate(['rock','paper','scissor'])}
    print("Enter rock/paper/scissor")
    while True:
        p1 = input("Player 1 -> ").strip().lower()
        if p1 not in ('rock','paper','scissor'):
            print("Input not in ('rock','paper','scissor')")
            continue
        else:
            break

    while True:
        p2 = input("Player 2 -> ").strip().lower()
        if p2 not in ('rock','paper','scissor'):
            print("Input not in ('rock','paper','scissor')")
            continue
        else:
            break

    print("winner is : ", winner(options[p1], options[p2]).upper())
