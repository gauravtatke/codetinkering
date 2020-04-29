#!/usr/bin/env python3

# Given two polynomial numbers represented by a linked list. The task is
# to complete the  function addPolynomial that adds these lists meaning
# adds the coefficients who have same variable powers.


class SllNode:
    def __init__(self, coeff=None, power=None, np=None):
        self.coeff = coeff
        self.power = power
        self.np = np

    def __str__(self):
        return str(self.coeff) + 'X^' + str(self.power)


def createlist(alist):
    head = None
    tail = None
    for coef, pw in alist:
        if tail:
            tail.np = SllNode(coef, pw)
            tail = tail.np
        else:
            head = tail = SllNode(coef, pw)
    return head


def printlist(head):
    curr = head
    strl = ''
    while curr:
        strl = strl + str(curr) + ' + '
        curr = curr.np
    print(strl.strip(' +'))


def addPoly(head1, head2):
    head = None
    tail = None
    while head1 and head2:
        if head1.power > head2.power:
            if tail:
                tail.np = head1
                tail = tail.np
            else:
                head = tail = head1
            head1 = head1.np
        elif head2.power > head1.power:
            if tail:
                tail.np = head2
                tail = tail.np
            else:
                head = tail = head2
            head2 = head2.np
        else:
            head1.coeff = head1.coeff + head2.coeff
            tail.np = head1
            tail = tail.np
            head1 = head1.np
            head2 = head2.np

    if head1:
        # if head1 still has something, append it to end of tail
        tail.np = head1
    elif head2:
        tail.np = head2

    return head


def main():
    alist1 = [(5, 2), (4, 1), (2, 0)]
    alist2 = [(5, 1), (5, 0)]
    head1 = createlist(alist1)
    head2 = createlist(alist2)
    printlist(head1)
    printlist(head2)
    printlist(addPoly(head1, head2))


if __name__ == '__main__':
    main()
