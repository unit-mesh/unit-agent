import {execa} from 'execa';

async function main() {
    const child = execa('target/debug/ulsp', [], {});

    child.on('close', (data) => {
        console.log(data);
    });

    child.stdout.setEncoding('utf8');
    child.stderr.setEncoding('utf8');

    child.stdout.on('data', function (raw) {
        console.log(raw);
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

    child.stdin.write(`${JSON.stringify({
        method: 'initialize',
    })}`);

    return child;
}

await main().then((child) => {
    console.log(child)
});