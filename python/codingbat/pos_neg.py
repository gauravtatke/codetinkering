def pos_neg(a, b, negative):
    if negative == False:
        if (a < 0 and b > 0) or (a > 0 and b < 0):
            return True
        else:
            return False
    elif a < 0 and b < 0:
        return True
    else:
        return False

if __name__ == "__main__":
    print(pos_neg(-1, -5, True))
