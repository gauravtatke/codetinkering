// process.env.UV_THREADPOOL_SIZE = 1;

const crypto = require('crypto');
const http = require('https');
const fs = require('fs');

const start = Date.now();

function doHash() {
    crypto.pbkdf2('a', 'b', 100000, 512, 'sha512', () => {
        console.log('Hash: ', Date.now() - start)
    });
}

function doRequest() {
    http.request('https://www.google.com', res => {
        res.on('data', () => {});
        res.on('end', ()=> {
            console.log('Http: ', Date.now() - start);
        });
    }).end();
}

doRequest();

fs.readFile('multitask.js', 'utf8', () => {
    console.log('FS: ', Date.now() - start);
})

doHash();
doHash();
doHash();
doHash();