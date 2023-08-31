// needed to access tauri
const { invoke } = window.__TAURI__.tauri

invoke('get_ip_address').then(ip => {
    document.getElementById("ip").innerText = ip
});

function sendMessage() {
    let message = document.getElementById("messageText").value + "\n";
    invoke('send_message', {message: message, ip: document.getElementById("outgoingIp").value});
    document.getElementById("receivedMessages").innerText += "Me: " + message;
}

function receiveMessages() {
    document.getElementById("receiveMessagesButton").disabled = true
        invoke('receive_messages').then( messages => {
            document.getElementById("receivedMessages").innerText += messages;
            receiveMessages()
    });
}