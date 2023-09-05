<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { ref } from 'vue';
import { Message, MessageType } from '../Message'

let encryptedIp = ref('');
let username = ref('');

function connect(encryptedIp: string) {
    invoke('get_encrypted_ip_address').then( (ip: any) => {

    let message = JSON.stringify(
        Message(ip, username.value, MessageType.ConnectionRequest, "")
    );
    invoke('send_message_encrypted_ip', {message: message, encryptedIp: encryptedIp})
})
}

</script>

<template>
    <h1>3D Tic Tac Toe</h1>
    <div class="center">
        <input v-model="username" placeholder="Username...">
        <input v-model="encryptedIp" placeholder="Enter Lobby Id...">
        <button @click="connect(encryptedIp)">Join</button>
    </div>
    <router-link class="center" to="CreateLobby">Create Lobby</router-link>
</template>

<style scoped >
    .center {
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>