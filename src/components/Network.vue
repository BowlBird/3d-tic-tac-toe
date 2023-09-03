<script setup lang="ts">
// needed to access tauri
import { invoke } from '@tauri-apps/api'
import { ref } from 'vue';

let localIp = ref('')
let remoteIp = ref('')
let outgoingMessage = ref('')
let messageHistory = ref('')

invoke('get_encrypted_ip_address').then((ip: any) => {
    localIp.value = ip
});

receiveMessage()

function sendMessage() {
    let message = `${outgoingMessage.value}\n`;
    invoke('send_message_encrypted_ip', {message: message, encryptedIp: remoteIp.value});
    messageHistory.value += `You: ${message}`;
}

async function receiveMessage() {
    invoke('receive_message').then((messages: any) => {
        messageHistory.value += `${remoteIp.value}: ${messages}`;
        receiveMessage()
    });
} 
</script>

<template>
    <router-link to="/Test">Go to Test</router-link>
  <h1>{{ localIp }}</h1>
  <input v-model="remoteIp" placeholder="Send to...">
  <input v-model="outgoingMessage" placeholder="Message...">
  <button @click="sendMessage">Send</button>
  <pre>{{ messageHistory }}</pre>
</template>