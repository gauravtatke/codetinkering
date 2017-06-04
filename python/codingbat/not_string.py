def not_string(str1):
    if str1.startswith("not"):
        return str1
    return "not " + str1

if __name__ == "__main__":
    print(not_string("not candy"))
