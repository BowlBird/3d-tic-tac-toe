<script setup lang="ts">
// needed to access tauri
import { invoke } from '@tauri-apps/api'
import { ref } from 'vue';

let localIp = ref('')
let remoteIp = ref('')
let outgoingMessage = ref('')
let messageHistory = ref('')

invoke('get_ip_address').then((ip: any) => {
    localIp.value = ip
});
receiveMessages()

function sendMessage() {
    let message = `${outgoingMessage.value}\n`;
    invoke('send_message', {message: message, ip: remoteIp.value});
    messageHistory.value += `You: ${message}`;
}

function receiveMessages() {
    invoke('receive_messages').then((messages: any) => {
        console.log("HERE");
        messageHistory.value += `${remoteIp.value}: ${messages}`;
        receiveMessages()
    });
}
</script>

<template>
  <h1>{{ localIp }}</h1>
  <input v-model="remoteIp" placeholder="Send to...">
  <input v-model="outgoingMessage" placeholder="Message...">
  <button @click="sendMessage">Send</button>
  <pre>{{ messageHistory }}</pre>
</template>