import {execa} from 'execa';
import chalk from 'chalk';

function formatLog(msg) {
    const logPattern = /^\[(\d{4}-\d{2}-\d{2})]\[(\d{2}:\d{2}:\d{2})]\[(\w+)]\[(\w+)] (.*)$/;
    const match = logPattern.exec(msg);

    const date = match[1];
    const time = match[2];
    const logger = match[3];
    const level = match[4];
    const message = match[5];

    // console.log(chalk.blue(date), chalk.magenta(time), chalk.yellow(logger), chalk.green(level), message);
    // format the log message: [2023-05-22][23:31:20][unit_agent][INFO] Logging with fern is set up
    return `${chalk.blue(date)} ${chalk.magenta(time)} ${chalk.yellow(logger)} ${chalk.green(level)} ${message}`
}

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

                console.info(formatLog(msg));
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
