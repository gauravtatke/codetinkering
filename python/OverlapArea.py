import math

class Point:
    def __init__(self, x, y):
        self.x_co = x
        self.y_co = y
        #self.setquad()

    def length(self, pt):
        dist = math.sqrt(abs(pt.x_co - self.x_co)**2 + abs(pt.y_co - self.y_co)**2)
        return dist

    def __str__(self):
        return str((self.x_co, self.y_co))

class Rectangle:
    def __init__(self, c1, c2, c3, c4):
        self.setcoord(c1, c2, c3, c4)
        self.count = -1

    def setcoord(self, coord1, coord2, coord3, coord4):
        '''
        Set the co-ordinates in following order
        bottom_l, bottom_r, upper_r, upper_l
        traverse anti-clockwise
        '''
        coord = (coord1, coord2, coord3, coord4)
        minx = min(coord1.x_co, coord2.x_co, coord3.x_co, coord4.x_co)
        miny = min(coord1.y_co, coord2.y_co, coord3.y_co, coord4.y_co)
        for co in coord:
            if co.x_co == minx and co.y_co != miny:
                #it is upper left co-ordinate
                self.upper_l = co
            if co.x_co != minx and co.y_co != miny:
                #it is upper right co-ordinate
                self.upper_r = co
            if co.x_co != minx and co.y_co == miny:
                #it is bottom right coordinate
                self.bottom_r = co
            if co.x_co == minx and co.y_co == miny:
                #it is bottom left co-ordinate
                self.bottom_l = co

    def __str__(self):
        st = '['
        for pt in self.bottom_l, self.bottom_r, self.upper_r, self.upper_l:
            st = st + str(pt) + ','
        st = st.rstrip(',') + ']'
        return st

    def __contains__(self, pt):
        '''
        Accepts Point object
        Returns True if point lies within rectangle, False otherwise
        '''
        if not isinstance(pt, Point):
            raise TypeError
        if self.bottom_l.x_co <= pt.x_co <= se2w2lf.bottom_r.x_co and \
                self.bottom_l.y_co <= pt.y_co <= self.upper_l.y_co:
            return True
        return False

    def __iter__(self):
        return self

    def __next__(self):
        pts = (self.bottom_l, self.bottom_r, self.upper_r, self.upper_l)
        self.count += 1
        if self.count > 3:
            raise StopIteration
        return pts[self.count]

    @classmethod
    def overlap_rec(cls, rec1, rec2):
        btm_l_x = max(rec1.bottom_l.x_co, rec2.bottom_l.x_co)
        btm_l_y = max(rec1.bottom_l.y_co, rec2.bottom_l.y_co)

        up_r_x = min(rec1.upper_r.x_co, rec2.upper_r.x_co)
        up_r_y = min(rec1.upper_r.y_co, rec2.upper_r.y_co)

        return cls(Point(btm_l_x, btm_l_y), Point(up_r_x, up_r_y), Point(btm_l_x, up_r_y), \
                Point(up_r_x, btm_l_y))



    def area(self):
        length = abs(self.bottom_r.x_co - self.bottom_l.x_co)
        width = abs(self.upper_r.y_co - self.bottom_r.y_co)
        return length * width

    @classmethod
    def calc_overlap_area(cls, rec1, rec2):
        overec = cls.overlap_rec(rec1, rec2)
        #print(overec)
        if overec.bottom_l.x_co < overec.bottom_r.x_co and \
                overec.bottom_r.y_co < overec.upper_r.y_co:
            return overec.area()
        return 0

if __name__ == '__main__':
    rec1 = Rectangle(Point(2,6), Point(6,3), Point(6,6), Point(2,3))
    rec2 = Rectangle(Point(4,5), Point(4,9), Point(11,5), Point(11,9))
    rec3 = Rectangle(Point(4, 5), Point(4, 6), Point(6,6), Point(6,4))
    rec4 = Rectangle(Point(10, 5), Point(20, 5), Point(20, 20), Point(10, 20))
    print(rec3)
    #print('Overlap Area = ', Rectangle.calc_overlap_area(rec1, rec2))
    #print('Area = ', rec3.area())

