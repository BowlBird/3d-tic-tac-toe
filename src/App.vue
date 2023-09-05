<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { Ref, ref } from 'vue';
invoke('message_watcher');
receiveMessage();

async function receiveMessage() {
    invoke('receive_message').then((message: unknown) => {
      messages.value.push(message as string);
      console.log(messages.value);
        //let message = JSON.parse(json as string);
        //messageHistory.value += `${message.username}: ${message.content}\n`;
        receiveMessage()
    });
} 

let messages: Ref<String[]> = ref([])
</script>

<template>
   <router-view />
</template>