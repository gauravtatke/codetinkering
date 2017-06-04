class TestIter:
    def __init__(self, maxval):
        self.maxval = maxval
        self.count = 0

    def __iter__(self):
        return self

    def __next__(self):
        if self.count == self.maxval:
            raise StopIteration
        self.count += 1
        return self.count

if __name__ == '__main__':
    tiobj = TestIter(5)
    print(tiobj.maxval)
    it = iter(tiobj)
    print(it)
    print(next(it))
    print(next(it))
    print(next(it))
    print(next(it))
