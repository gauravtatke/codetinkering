#!/usr/etc/ python

from socket import *
from time import ctime

host = ''
port = 21567
bufsize = 1024

addr = (host,port)

tcpServSock = socket(AF_INET,SOCK_STREAM)
tcpServSock.bind(addr)
tcpServSock.listen(5)

while True:
    print "Waiting for connections..."
    tcpCliSock,addrc = tcpServSock.accept()
    print ".. Connected from : ",addr
    while True:
        data = tcpCliSock.recv(bufsize)
        if not data:
            break
        tcpCliSock.send('[%s] %s' %(ctime(),data))
    tcpCliSock.close()
