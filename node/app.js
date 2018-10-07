// const Emitter = require('./emitter');
const Emitter = require('events')
const eventConfig = require('./config').events;

const emt = new Emitter();

emt.on(eventConfig.GREET, function(){
    console.log('someone, somewhere said hello!!');
});

emt.on(eventConfig.GREET, function(){
    console.log('A greeting occured');
});

console.log('Hello!');

emt.emit(eventConfig.GREET);