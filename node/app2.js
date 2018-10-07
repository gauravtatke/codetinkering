'use strict'

const EventEmitter = require('events');

class Greeter extends EventEmitter {
    constructor() {
        super();
        this.greeting = 'Hello, World!';
    }

    greet(data) {
        console.log(`${this.greeting}: ${data}`);
        this.emit('greet', data);
    }
}

const greetr = new Greeter();

greetr.on('greet', function(data){
    console.log(`Someone greeted - ${data}`);
});

greetr.greet('jason');
