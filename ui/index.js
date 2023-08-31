// needed to access tauri
const { invoke } = window.__TAURI__.tauri

invoke('get_ip_address').then(ip => {
    document.getElementById("ip").innerText = ip
});

function allowConnections() {
    invoke('allow_connections')
    document.getElementById("allowConnections").innerText = "Connections Allowed";
    //document.getElementById("allowConnectionsButton").disabled = true;
}

function connect() {
    invoke('connect', {ip: document.getElementById("connectToAddress").value})
    //document.getElementById("connectToAddress").disabled = true
    //document.getElementById("connectToAddressButton").disabled = true
}

function sendMessage() {
    let message = document.getElementById("messageText").value + "\n";
    invoke('send_message', {message: message});
    document.getElementById("receivedMessages").innerText += "Me: " + message;
}

function receiveMessages() {
    invoke('receive_messages').then( messages => {
        document.getElementById("receivedMessages").innerText += messages;
    });
}