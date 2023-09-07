<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { useCallbackStore } from './main.ts';
import { Message, messageFromString } from './components/Message';

invoke('message_watcher');
receiveMessage();

async function receiveMessage() {
   invoke('receive_message').then((messageString: unknown) => {
      let message = messageFromString(messageString as string)

      invoke('log', {string: `Sending '${messageString}' To: `})

      const callbacks = useCallbackStore();
      callbacks.functions.get(message.type)?.forEach((fun: ((message: Message) => void)) => {
         invoke('log', {string: fun.name.toString()})
         fun(message);
      })

      receiveMessage()
   });
} 
</script>

<template>
   <router-view />
</template>