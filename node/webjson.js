'use strict';

const http = require('http');

http.createServer(function(req, res) {
    res.writeHead(200, {'Content-Type': 'application/json'});
    const obj = {
        firstname: 'Gaurav',
        lastname: 'Tatke'
    }
    res.end(JSON.stringify(obj));

}).listen(1337, '127.0.0.1');