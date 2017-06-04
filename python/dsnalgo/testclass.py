class Human(object):
    def __init__(self):
        self.name = "gaurav"

    def getName(self):
        return self.name

class Emp(Human):
    def __init__(self):
        super(Emp, self).__init__()
        self.staff = 4375

    def getStaff(self):
        return self.staff

if __name__ == '__main__':
    e = Emp()
    print(e.getName())
    print(e.getStaff())
