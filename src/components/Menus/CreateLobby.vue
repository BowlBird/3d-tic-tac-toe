<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { ref } from 'vue';
import { useCallbackStore, sendMessage, useVarStore } from '../../main.ts';
import { Message, MessageType, createMessage } from '../Message';
import router from '../../router';

let localIp = ref('');
let messageHistory = ref('');

invoke('get_encrypted_ip_address').then((ip: any) => {
    localIp.value = ip;
});

const callbacks = useCallbackStore();
callbacks.register(MessageType.ConnectionRequest, onConnectionRequest)

function onConnectionRequest(message: Message) {
    createMessage(useVarStore().username, MessageType.ConnectionApproval, "").then((newMessage: unknown) => {
        sendMessage(newMessage as Message, message.from);
    })
}

function back() {
    callbacks.clear()
    router.push('Title')
}

</script>

<template>
    <h1>{{ localIp }}</h1>
    <pre>{{ messageHistory }}</pre>
    <div class="center">
        <button @click="back">Go Back</button>
    </div>
</template>
<style scoped >
    .center {
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>