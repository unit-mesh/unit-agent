import {execa} from 'execa';

async function main() {
    const child = execa('target/debug/unit_agent', [], {});

    child.on('close', (data) => {
        console.log(data);
    });

    child.stdout.setEncoding('utf8');
    child.stderr.setEncoding('utf8');

    child.stdout.on('data', function (raw) {
        parseMessages(raw).forEach((msg) => {
            console.log(msg);
        });
    });
    child.stderr.on('data', function (data) {
        console.log(`${data}`);
    });

    function parseMessages(raw) {
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

    console.log('Sending initialize message to core');

    function send(method, params, rest) {
        const data = {method, params, ...rest};
        try {
            child.stdin.write(`${JSON.stringify(data)}\n`);
            return true;
        } catch (e) {
            console.error(e);
            return false;
        }
    }

    // {"method":"client_started","params":{}}
    // https://github.com/phodal/stadal/blob/master/gui/src/render/core.ts#L52
    send('client_started', {});
    send('config', {});

    return child;
}

await main().then((child) => {
    console.log(child)
});