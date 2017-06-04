#! /usr/bin/env python

import re
import sys
import os

rgexp = "(\d{1,2}):(\d{1,2}):(\d{1,2}),(\d{1,3})" #regexp to search

def createNewFile(chprm,srcfile,trgtfile):
    #rh = open('/root/source.srt','r')
    #wh = open('/root/target.srt','w')
    try:
        rh = open(srcfile,'r')
    except IOError, e:
        print "Could not open file : ",e
    try:
        wh = open(trgtfile,'w')
    except IOError, e:
        print "Could not open file : ",e
    for eachLine in rh:
        chLine = ""
        flag = 0
        lst = re.findall(rgexp,eachLine)
        #print lst
        #print "inside for1"
        if len(lst) != 0:
            #print "inside if1"
            #if sys.argv[1] == '-i':
            for tpl in lst:
                flag = flag+1
                #print "inside for2"
                if sys.argv[1] == '-i':
                    chLine = chLine + incTime(tpl,chprm)
                else:
                    chLine = chLine + decTime(tpl,chprm)
                if flag == 1:
                    chLine = chLine + ' --> '
            wh.write(chLine+os.linesep)
            #print "written : ", chLine
        else:
            wh.write(eachLine)
            #print "written : " , eachLine
    wh.close()
    rh.close()

def decTime(timeToCh,chprm):
    hr = int(timeToCh[0])
    mins = int(timeToCh[1])
    sec = int(timeToCh[2])
    msec = int(timeToCh[3])
    nwln = ""
    if (msec-chprm[3]) < 0:
        msec = 1000 + msec - chprm[3]
        sec = sec - 1
        #print "if2"
    else:
        msec = msec - chprm[3]
        print "else2"
    if (sec - chprm[2]) < 0:
        sec = 60 + sec - chprm[2]
        mins = mins - 1
        #print "if3"
    else:
        sec = sec - chprm[2]
        #print "else3"
    if (mins - chprm[1]) < 0:
        mins = 60 + mins - chprm[1]
        hr = hr - 1
        #print "if4"
    else:
        mins = mins - chprm[1]
        #print "else4"
    if (hr - chprm[0]) < 0:
        print "Error:Value entered is greater than value in file"
        #print "if5"
        sys.exit()
    else:
        hr = hr - chprm[0]
        #print "else5"
    #print "hr-%s, mins-%s, sec-%s, msec-%s" %(hr,mins,sec,msec)
    nwln = nwln+"%02d" %(hr)+':'+"%02d" %(mins)+':'+"%02d" %(sec)+','+"%03d" %(msec)
    return nwln


def incTime(timeToCh,chprm):
    hr = int(timeToCh[0])
    mins = int(timeToCh[1])
    sec = int(timeToCh[2])
    msec = int(timeToCh[3])
    nwln = ""
    if msec+chprm[3]>999:
        msec = msec + chprm[3] - 1000
        sec = sec + 1
    else:
        msec = msec + chprm[3]
    if sec+chprm[2] > 59:
        sec = sec + chprm[2] - 60
        mins = mins + 1
    else:
        sec = sec + chprm[2]
    if mins+chprm[1] > 59:
        mins = mins + chprm[1] - 60
        hr = hr + 1
    else:
        mins = mins + chprm[1]
    hr = hr + chprm[0]
    #print "hr-%s, mins-%s, sec-%s, msec-%s" %(hr,mins,sec,msec)
    nwln = nwln+"%02d" %(hr)+':'+"%02d" %(mins)+':'+"%02d" %(sec)+','+"%03d" %(msec)
    return nwln


if __name__ == '__main__':
    print "Arguements %d : %s" %(len(sys.argv),str(sys.argv))
    if (len(sys.argv)!=4) or (sys.argv[1]!='-i' and sys.argv[1]!='-d') or not os.path.isfile(sys.argv[3]):
        print "Usage: subsync.py -i[--increase]|-d[--decrease] HH:MM:SS,mmm[--hour:min:sec,millisec] srcfile[--path/srcfile]"
        print "Make sure path to source exists and it is a file not a directory"
        print "for e.g. subsync.py -d 00:03:02,001 /home/gaurav/source.srt"
        sys.exit()
    else:
        retobj = re.match(rgexp,sys.argv[2])
        if retobj is not None:
            chparam = list(retobj.groups())
            for i in range(4):
                chparam[i] = int(chparam[i])
            #print "All meaningful work should go here"
            currdir = os.getcwd()
            srcfile = os.path.join(currdir,sys.argv[3])
            trgtdir = os.path.dirname(srcfile)
            trgtfile = os.path.join(trgtdir,os.path.basename(srcfile)+'_target')
            createNewFile(chparam,srcfile,trgtfile)
        else:
            print "Incorrect change-param:%s, should be in format HH:MM:SS,mmm" %str(sys.argv[2])
