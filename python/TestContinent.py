#/usr/bin/env python3

import unittest
from continent import pathExists

class ContinentTest(unittest.TestCase):
    def setUp(self):
        self.arr1 = [ 
                [1, 1, 1, 1, 0, 1],
                [0, 1, 0, 1, 0, 0],
                [1, 1, 1, 1, 1, 1],
                [0, 0, 0, 1, 0, 1],
                [1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 1]
            ]

        self.arr2 = [
                [1, 1, 1],
                [0, 1, 0],
                [1, 1, 1],
            ]

    # def test1_pathExists(self):
        # arr = self.arr2
        # visited = [[False for co in arr[0]] for ro in arr]

        # self.assertTrue(pathExists(arr, visited, (0,2), (1,1)))

    # def test2_pathExists(self):
        # arr = self.arr2
        # visited = [[False for co in arr[0]] for ro in arr]
        # self.assertTrue(pathExists(arr, visited, (2,2), (0,0)))

    # def test3(self):
        # arr = self.arr2
        # visited = [[False for co in arr[0]] for ro in arr]
        # self.assertTrue(pathExists(arr, visited, (2,0), (0,2)))

    # def test4(self):
        # arr = self.arr2
        # visited = [[False for co in arr[0]] for ro in arr]
        # self.assertTrue(pathExists(arr, visited, (2,0), (0,0)))

    # def test5(self):
        # arr = self.arr2
        # visited = [[False for co in arr[0]] for ro in arr]
        # self.assertFalse(pathExists(arr, visited, (2,2), (1,2)))
    
    # def test6(self):
        # arr = self.arr2
        # visited = [[False for co in arr[0]] for ro in arr]
        # self.assertFalse(pathExists(arr, visited, (2,2), (1,0)))

    # def test7(self):
        # arr = self.arr2
        # visited = [[False for co in arr[0]] for ro in arr]
        # self.assertFalse(pathExists(arr, visited, (0,2), (1,0)))

    def test8(self):
        arr = self.arr1
        visited = [[False for co in arr[0]] for ro in arr]
        self.assertTrue(pathExists(arr, visited, (1,1), (0,2)))

    # def test9(self):
        # arr = self.arr1
        # visited = [[False for co in arr[0]] for ro in arr]
        # self.assertTrue(pathExists(arr, visited, (0,0), (5,5)))

    # def test10(self):
        # arr = self.arr1
        # visited = [[False for co in arr[0]] for ro in arr]
        # self.assertFalse(pathExists(arr, visited, (0,0), (0,5)))
    
    # def test11(self):
        # arr = self.arr1
        # visited = [[False for co in arr[0]] for ro in arr]
        # self.assertFalse(pathExists(arr, visited, (5,5), (0,5)))

    def tearDown(self):
        pathExists.result = False

if __name__ == '__main__':
    unittest.main()
