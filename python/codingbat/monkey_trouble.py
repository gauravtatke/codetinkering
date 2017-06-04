def monkey_trouble(a_smile, b_smile):
    if a_smile and b_smile:
        return True
    elif not a_smile and not b_smile:
        return True
    else:
        return False

if __name__ == "__main__":
    print(monkey_trouble(False, True))
