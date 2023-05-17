const {spawn} = require('node:child_process');
const rpc = require('vscode-jsonrpc/node');

let childProcess = spawn('target/debug/ulsp', []);

childProcess.stdout.on('data', function (data) {
    console.log(data.toString());
});

childProcess.stdin.on('data', function (data) {
    console.log(data.toString());
});

let connection = rpc.createMessageConnection(
    new rpc.StreamMessageReader(childProcess.stdout),
    new rpc.StreamMessageWriter(childProcess.stdin)
);

connection.listen();
connection.sendRequest(new rpc.NotificationType('initialize'), {
    "capabilities": {
        "textDocument": {"synchronization": {"dynamicRegistration": true}},
        "workspace": {"isWorkspaceFolders": true}
    },
    "workspaceFolders": [{
        "uri": "file:///Volumes/source/ai/gpt-samples/ddd-monolithic-code-sample"
    }]
});

// connection.sendRequest(new rpc.NotificationType('setEditorInfo'), {
//     "editorInfo": {
//         "name": "JetBrains-IC",
//         "version": "222.3345.118"
//     },
//     "editorPluginInfo": {"name": "copilot-intellij", "version": "0.2.0"},
//     "editorConfiguration": {"showEditorCompletions": false, "enableAutoCompletions": true, "disabledLanguages": []}
// });
