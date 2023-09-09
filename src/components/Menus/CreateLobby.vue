<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { Ref, ref } from 'vue';
import { useCallbackStore, sendMessage, useVarStore } from '../../main.ts';
import { Message, MessageType, createMessage } from '../Message';
import router from '../../router';

let localIp = ref('');
//DOES NOT INCLUDE HOST
let connectedPlayers: Ref<Player[]> = ref([])
let runHeartbeat = true

invoke('get_encrypted_ip_address').then((ip: any) => {
    localIp.value = ip;
});

const callbacks = useCallbackStore();
callbacks.register(MessageType.ConnectionRequest, onConnectionRequest)
callbacks.register(MessageType.LobbyInitialInformationRequest, respondInitialInfo)
callbacks.register(MessageType.LobbyClientDisconnectionNotification, onDisconnectionNotification)

sendHeartbeat()
updateInfo()

type Player = {
    username: string;
    ip: string
}


function onConnectionRequest(message: Message) {
    connectedPlayers.value.push({
        username: message.username,
        ip: message.from
    })
    createMessage(useVarStore().username, MessageType.ConnectionApproval, "").then((newMessage: unknown) => {
        sendMessage(newMessage as Message, message.from);
    })
}

async function sendHeartbeat() {
    while(runHeartbeat) {
        for(let player of connectedPlayers.value) {
            createMessage(useVarStore().username, MessageType.HeartbeatCall, "").then((message: Message) => {
                sendMessage(message, player.ip)
            })
        }

        //listening logic

        //copy people in lobby to new array
        let notResponded: string[] = []
        connectedPlayers.value.forEach((player: Player) => notResponded.push(player.ip))

        //on response
        let listen = ((message: Message) => {
            const index = notResponded.indexOf(message.from, 0);
            if (index > -1) {
                notResponded.splice(index, 1);
            }
        })
        callbacks.register(MessageType.HeartbeatResponse,listen)
        await new Promise(f => setTimeout(f, 3000));
        callbacks.deregister(MessageType.HeartbeatResponse, listen)

        //remove inactive players
        for (let inactivePlayer of notResponded) {
            //remove from heartbeat
            connectedPlayers.value = connectedPlayers.value.filter((player: Player) => {
                return player.ip !== inactivePlayer
            })
        }
    }
}

function respondInitialInfo(message: Message) {
    let usernames = connectedPlayers.value.map((player: Player) => {
        return player.username
    })
    usernames.push(useVarStore().username)

    createMessage(useVarStore().username, MessageType.LobbyInitialInformationResponse, `{"players":"${usernames}"}`).then((innerMessage: Message) => {
        sendMessage(innerMessage, message.from)
    })
}

async function updateInfo() {
    while(runHeartbeat) {
        let usernames = connectedPlayers.value.map((player: Player) => {
            return player.username
        })
        usernames.push(useVarStore().username)
        for (let player of connectedPlayers.value) {
            createMessage(useVarStore().username, MessageType.LobbyInformationUpdate, `{"players":"${usernames}"}`).then((innerMessage: Message) => {
                sendMessage(innerMessage, player.ip)
            })
        }
        await new Promise(f => setTimeout(f, 3000));
    }
}

function onDisconnectionNotification(message: Message) {
    connectedPlayers.value = connectedPlayers.value.filter((player: Player) => {
        return player.ip !== message.from
    })
}

function back() {
    runHeartbeat = false
    callbacks.clear()
    router.push('Title')
    createMessage(useVarStore().username, MessageType.LobbyHostDisconnectionNotification, "").then ((message: Message) => {
        connectedPlayers.value.forEach((player: Player) => {
            sendMessage(message, player.ip)
        })
    })
}

</script>

<template>
    <h1>{{ localIp }}</h1>
    <li class="center">
        {{ useVarStore().username }}
    </li>
    <li class="center" v-for="player in connectedPlayers">
        {{ player.username }}
    </li>
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