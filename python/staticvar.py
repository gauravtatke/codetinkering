#!/usr/bin/env python3

class Foo():
    counter = 0

    def __call__(self):
        self.counter += 1
        print(self.counter)

