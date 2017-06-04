def sleep_in(weekday, vacation):
    if weekday == False or vacation == True:
        return True
    else:
        return False


if __name__ == "__main__":
    print(sleep_in(weekday=False, vacation=True))
