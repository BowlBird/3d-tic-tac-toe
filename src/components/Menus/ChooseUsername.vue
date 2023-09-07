<script setup lang="ts">
import { ref } from 'vue';
import router from "../../router";
import { useVarStore } from "../../main"
import { invoke } from '@tauri-apps/api';


let username = ref('')

function accept(username: string) {
    useVarStore().username = username
    invoke('log', {string: `Set Username To: ${username}`})
    router.push('Title');
}

</script>
<template>
<h1 class="center">Please Enter Your:</h1>
<div class="center">
    <input placeholder="Username..." v-model="username" v-on:keydown.enter="accept(username)">
    <button @click="accept(username)">Accept</button>
</div>
</template>
<style scoped >
    .center {
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>