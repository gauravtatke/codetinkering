import urllib.request
import urllib.parse
import urllib.error
import json

import metricdata

tsd_url = 'http://localhost:4242'

def create_metric(metdict, *mor_metdict):
    api = '/api/uid/assign'
    global tsd_urla
    base_url = tsd_url + api
    url = base_url + '?' + querystring(metdict)
    
    #payload = json.dumps(metdict)
    #payload = payload.encode('utf-8')
    #header = {'content-type': 'application/json'}
    #req = urllib.request.Request(url, data=payload, headers=header)
    try:
        response = urllib.request.urlopen(url)
        for more in mor_metdict:
            url = base_url + '?' + querystring(more)
            response = urllib.request.urlopen(url)
    except urllib.error.HTTPError as he:
        #print(response)
        if he.code == 400:
            #use he.read() on few hundred bytes and check for errors using
            #regular expression
            pass
        else:
            raise

    #print(response)
    #for payload in mor_metdict:
    #    met_data = urllib.parse.urlencode(dict(metdict))
    #    req = urllib.request.Request(url, data=met_data,\
    #                                 headers=header)
    #    response = urllib.request.urlopen(req)
    #    print(response) 

def querystring(adict):
    mstr = 'metric='
    tkstr = 'tagk='
    tvstr = 'tagv='
    mlist = adict['metric']
    tklist = adict['tagv']
    tvlist = adict['tagk']

    qstr = mstr + ','.join(mlist) 
    qstr = qstr + '&' + tkstr + ','.join(tkstr) + tvstr + ','.join(tvstr)
    return qst

def senddata():
    pass

if __name__ == '__main__':
    for m in metricdata.allmet():
        res = create_metric(m)
    #print(res)
