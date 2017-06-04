'''This file contains variables for metrics and a tuple which contains
metric variable names'''

metric1 = {
            'metric': ['cali.temp', 'cali.humid', 'cali.precip'],
            'tagk': ['city'],
            'tagv': ['sanfranc', 'sandiego', 'losangel']
        }

_metlist = (metric1,)

def allmet():
    global _metlist
    for met in _metlist:
        yield met

if __name__ == '__main__':
    print(type(metric1))
    #print(_metlist)
    for m in allmet():
        print(m)
