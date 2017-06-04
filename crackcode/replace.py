
def replace(origstr, repch, withch):
    alist = []
    for ch in origstr:
        if ch == repch:
            alist.append(withch)
        else:
            alist.append(ch)
    return ''.join(alist)

if __name__ == '__main__':
    print('replace string = ', replace('gaurav tatke is sexy', ' ', '%20'))
    print('%20'.join('gaurav tatke is sexy'.split(' ')))
