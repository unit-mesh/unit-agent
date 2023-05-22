import {execa} from 'execa';

class RpcClient {
    constructor() {
        this.child = execa('target/debug/unit_agent', [], {});
        this.child.on('close', (data) => {
            console.log(data);
        });
        this.child.stdout.setEncoding('utf8');
        this.child.stderr.setEncoding('utf8');

        this.child.stdout.on('data', (raw) => {
            this.parse(raw).forEach((msg) => {
                console.log(msg);
            });
        });

        this.child.stderr.on('data', (raw) => {
            raw.split('\n').forEach((msg) => {
                if (msg === '') {
                    return;
                }

                console.info(msg);
            });
        });
    }

    parse(raw) {
        const parsed = [];
        const lines = raw.split('\n');
        for (let i = 0; i < lines.length; ++i) {
            if (typeof lines[i] !== 'string' || lines[i] === '') {
                continue;
            }
            try {
                parsed.push(JSON.parse(lines[i]));
            } catch (err) {
                console.warn('Error parsing message from core!');
                console.error(err);
            }
        }
        return parsed;
    }

    send(method, params, rest) {
        const data = {method, params, ...rest};
        try {
            this.child.stdin.write(`${JSON.stringify(data)}\n`);
            return true;
        } catch (e) {
            console.error(e);
            return false;
        }
    }

    start() {
        this.send('client_started', {});
        this.send('config', {"open_ai_token": "sk-xxx"}, {id: 0});
    }
}

const client = new RpcClient();
client.start();
