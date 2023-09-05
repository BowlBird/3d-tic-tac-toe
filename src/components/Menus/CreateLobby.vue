<script setup lang="ts">
// needed to access tauri
import { invoke } from '@tauri-apps/api'
import { ref } from 'vue';

let localIp = ref('');
let remoteIp = ref('');
let username = ref('');
let outgoingMessage = ref('');
let messageHistory = ref('');

invoke('get_encrypted_ip_address').then((ip: any) => {
    localIp.value = ip;
});

//receiveMessage()

function sendMessage(message: any, encryptedIp: string) {
    let messageString = JSON.stringify(message);
    invoke('send_message_encrypted_ip', {message: messageString, encryptedIp: encryptedIp});
    messageHistory.value += `You: ${message.content}\n`;
}
</script>

<template>
    <router-link to="/Test">Go to Test</router-link>
    <h1>{{ localIp }}</h1>
    <input v-model="remoteIp" placeholder="Send to...">
    <input v-model="username" placeholder="Username">
    <input v-model="outgoingMessage" placeholder="Message...">
    <button @click="sendMessage(
        {
            'from': localIp, 
            'username': username,
            'message': outgoingMessage
        },
        remoteIp)">Send</button>
    <pre>{{ messageHistory }}</pre>
</template>