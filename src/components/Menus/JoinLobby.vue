<script setup lang="ts">
import { Ref, ref } from 'vue';
import { sendMessage, useCallbackStore, useVarStore } from '../../main';
import router from '../../router';
import { Message, MessageType } from '../Message';
import { createMessage } from '../Message';

let connectedPlayers: Ref<string[]> = ref([])

let callbacks = useCallbackStore()
callbacks.register(MessageType.HeartbeatCall, heartbeatResponse)
callbacks.register(MessageType.LobbyInitialInformationResponse, handleInfoUpdate)
callbacks.register(MessageType.LobbyInformationUpdate, handleInfoUpdate)
callbacks.register(MessageType.LobbyHostDisconnectionNotification, onHostDisconnection)

requestInitialInfo()

function back() {
    useCallbackStore().clear()
    createMessage(useVarStore().username, MessageType.LobbyClientDisconnectionNotification, "").then((message: Message): void => {
        sendMessage(message, useVarStore().hostIp)
    })
    router.push('Title')
}

function requestInitialInfo() {
    createMessage(useVarStore().username, MessageType.LobbyInitialInformationRequest, "").then((message: Message) => {
        sendMessage(message, useVarStore().hostIp)
    })
}

function handleInfoUpdate(message: Message) {
    let response = JSON.parse(message.content)
    connectedPlayers.value = response.players.split(",")
}

function heartbeatResponse(message: Message) {
    createMessage(useVarStore().username, MessageType.HeartbeatResponse, "").then((innerMessage: Message) => {
        sendMessage(innerMessage, message.from)
    })
}

function onHostDisconnection(_: Message) {
    callbacks.clear()
    router.push("Title")
}

</script>

<template>
    <p>Lobby</p>
    <li class="center" v-for="player in connectedPlayers">
        {{ player }}
    </li>
    <div class="center">
        <button @click="back">Go Back</button>
    </div>
</template>