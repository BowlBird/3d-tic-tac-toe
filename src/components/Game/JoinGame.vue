<script setup lang="ts">
import { Ref, ref } from 'vue';
import { useCallbackStore, sendMessage, useVarStore, Player, PlayerInfo } from '../../main.ts';
import { Message, MessageType, createMessage } from '../Message';
import { invoke } from '@tauri-apps/api';


let callbacks = useCallbackStore()
callbacks.register(MessageType.GameStateUpdate, onGameStateUpdate)

let board: Ref<string[][][]> = ref([])
let players: Ref<PlayerInfo[]> = ref([])
let position = ref([0,0,0])
type GameState = {
    players: PlayerInfo[],
    board: string[][][]
}

function onGameStateUpdate(message: Message) {
    let state = JSON.parse(message.content) as GameState
    board.value = state.board
    players.value = state.players
}

window.onkeydown = (ev: KeyboardEvent): any => {
    // for correct array indexing, needs to follow
    //z, -y, x pattern
    switch(ev.key) {
        case "ArrowRight":
        case "l":
            if (position.value[2] < 3)    
                position.value[2] += 1
            break;
        case "ArrowLeft":
        case "h":
            if (position.value[2] > 0)
                position.value[2] -= 1
            break;
        case "ArrowUp":
        case "k":
            if (position.value[1] > 0)
                position.value[1] -= 1
            break;
        case "ArrowDown":
        case "j":
            if (position.value[1] < 3)
                position.value[1] += 1
            break;
        case "Tab":
            if (position.value[0] == 3)
                position.value[0] = 0
            else
                position.value[0] += 1
            break;
        case "Unidentified":
            if (position.value[0] == 0)
                position.value[0] = 3
            else
                position.value[0] -= 1
            break;
        case " ":
        case "Enter":
            createMessage(useVarStore().username, MessageType.GameTurnAttempt, JSON.stringify({position: position.value, icon: useVarStore().icon})).then((message: Message) => {
                sendMessage(message, useVarStore().hostIp)
            })
            break;
        default:
            break;

    }
    invoke('log', {string: ev.key.toString()})
    invoke('log', {string: position.toString()})
}

</script>
<template>
    <li class="center" v-for="player in players">
        {{ player.username }}
    </li>
<div class="center">
    <div class="board" v-for="(slice, z) in board">
        <tr class="slice" v-for="(row, y) in slice">
            <td class="slice" :class="{highlight: x == position[2] && y == position[1] && z == position[0]}" v-for="(char, x) in row">
            {{ char }}
            </td>
        </tr>     
    </div>   
</div>
</template>
<style scoped >
    .center {
        display: flex;
        justify-content: center;
        align-items: center;
    }
    .board {
        display: inline-block;
        margin: 2rem;
    }
    .center {
        display: flex;
        justify-content: center;
        align-items: center;
    }
    .slice {
        border: 1px solid black;
        padding: .5rem;
    }
    .highlight {
        border: 2px solid red;
    }
</style>