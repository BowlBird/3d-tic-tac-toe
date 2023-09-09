<script setup lang="ts">
import { ref } from 'vue';
import { createMessage, Message, MessageType } from '../Message'
import { sendMessage, useCallbackStore, useVarStore } from '../../main';
import router from '../../router';

let encryptedIp = ref('');

function connect(encryptedIp: string) {
    createMessage(useVarStore().username, MessageType.ConnectionRequest, "").then((message: unknown) => {
        sendMessage(message as Message, encryptedIp)
        useCallbackStore().register(MessageType.ConnectionApproval, onConnectionApproval)
    })
}

function onConnectionApproval(message: Message) {
    useVarStore().hostIp = message.from
    router.push('JoinLobby');
}

</script>

<template>
    <p class="center">Welcome to</p>
    <h1 class="center">3D Tic Tac Toe</h1>
    <router-link to="/" style="font-weight: bold; color: white" class="center">{{ useVarStore().username }}</router-link>
    <br>
    <br>
    <div class="center">
        <input v-model="encryptedIp" placeholder="Enter Lobby Id..." v-on:keydown.enter="connect(encryptedIp)">
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