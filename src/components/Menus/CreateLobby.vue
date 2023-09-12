<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { ref } from 'vue';
import { useCallbackStore, sendMessage, useVarStore, Player } from '../../main.ts';
import { Message, MessageType, createMessage } from '../Message';
import router from '../../router';



let localIp = ref('');
//DOES NOT INCLUDE HOST

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

let icons = ["O", "W", "F", "B", "A"]
useVarStore().icon = "X"

function onConnectionRequest(message: Message) {
    useVarStore().connectedPlayers.push({
        username: message.username,
        ip: message.from,
        icon: icons[useVarStore().connectedPlayers.length]
    })
    createMessage(useVarStore().username, MessageType.ConnectionApproval, "").then((newMessage: unknown) => {
        sendMessage(newMessage as Message, message.from);
    })
}

async function sendHeartbeat() {
    while(runHeartbeat) {
        for(let player of useVarStore().connectedPlayers) {
            createMessage(useVarStore().username, MessageType.HeartbeatCall, "").then((message: Message) => {
                sendMessage(message, player.ip)
            })
        }

        //listening logic

        //copy people in lobby to new array
        let notResponded: string[] = []
        useVarStore().connectedPlayers.forEach((player: Player) => notResponded.push(player.ip))

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
            useVarStore().connectedPlayers = useVarStore().connectedPlayers.filter((player: Player) => {
                return player.ip !== inactivePlayer
            })
        }
    }
}

function respondInitialInfo(message: Message) {
    let usernames = useVarStore().connectedPlayers.map((player: Player) => {
        return player.username
    })
    usernames.push(useVarStore().username)

    createMessage(useVarStore().username, MessageType.LobbyInitialInformationResponse, `{"players":"${usernames}"}`).then((innerMessage: Message) => {
        sendMessage(innerMessage, message.from)
    })
}

async function updateInfo() {
    while(runHeartbeat) {
        let usernames = useVarStore().connectedPlayers.map((player: Player) => {
            return player.username
        })
        usernames.push(useVarStore().username)
        for (let player of useVarStore().connectedPlayers) {
            createMessage(useVarStore().username, MessageType.LobbyInformationUpdate, `{"players":"${usernames}"}`).then((innerMessage: Message) => {
                sendMessage(innerMessage, player.ip)
            })
        }
        await new Promise(f => setTimeout(f, 3000));
    }
}

function onDisconnectionNotification(message: Message) {
    useVarStore().connectedPlayers = useVarStore().connectedPlayers.filter((player: Player) => {
        return player.ip !== message.from
    })
}

function back() {
    runHeartbeat = false
    callbacks.clear()
    router.push('Title')
    createMessage(useVarStore().username, MessageType.LobbyHostDisconnectionNotification, "").then ((message: Message) => {
        useVarStore().connectedPlayers.forEach((player: Player) => {
            sendMessage(message, player.ip)
        })
    })
}

function start() {
    callbacks.deregister(MessageType.ConnectionRequest, onConnectionRequest)
    callbacks.deregister(MessageType.LobbyInitialInformationRequest, respondInitialInfo)
    useVarStore().connectedPlayers.forEach((player: Player) => {
        createMessage(useVarStore().username, MessageType.GameStartNotification, player.icon).then((message: Message) => {
            sendMessage(message, player.ip)
        })
    })

    router.push('HostGame')
}

</script>

<template>
    <h1>{{ localIp }}</h1>
    <li class="center">
        {{ useVarStore().username }}
    </li>
    <li class="center" v-for="player in useVarStore().connectedPlayers">
        {{ player.username }}
    </li>
    <div class="center">
        <button @click="start">Start</button>
    </div>
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