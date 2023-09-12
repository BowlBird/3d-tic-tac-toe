<script setup lang="ts">
import { Ref, ref } from 'vue';
import { useCallbackStore, sendMessage, useVarStore, Player, PlayerInfo } from '../../main.ts';
import { invoke } from '@tauri-apps/api';
import { MessageType, createMessage, Message } from '../Message';

let callbacks = useCallbackStore()
callbacks.register(MessageType.GameTurnAttempt, onTurnAttempt)

//z, -y, x
let board = ref(constructArray(4,4,4))

//should follow z, -y, x pattern
let position = ref([0,0,0])

let players: PlayerInfo[] = useVarStore().connectedPlayers.map((player: Player) => {return {username: player.username, icon: player.icon}}) //copy
players.push({username: useVarStore().username, icon: "X"})
players = shuffleArray(players)
useVarStore().icon = "X"
sendGameState()

function shuffleArray<T>(array: T[]): T[] {
  const length = array.length;
  for (let i = length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [array[i], array[j]] = [array[j], array[i]];
  }
  return array;
}

function cycleArray<T>(array: T[]): T[] {
    if (array.length != 0) {
        let temp = array.splice(0,1)
        array.push(temp[0])
    }
    return array
}

function constructArray(xSize: number, ySize: number, zSize: number): String[][][] {
    let array: String[][][] = []
    for (let i = 0; i < zSize; i++) {
        let zArray: String[][] = []
        for (let j = 0; j < ySize; j++) {
            let yArray: String[] = []
            for (let k = 0; k < xSize; k++) {
                yArray.push(" ")
            }
            zArray.push(yArray)
        }
        array.push(zArray)
    }
    return array
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
                onTurnAttempt(message)
            })
            break;
        default:
            break;

    }
    invoke('log', {string: ev.key.toString()})
    invoke('log', {string: position.toString()})
}

type GameState = {
    players: PlayerInfo[],
    board: string[][][],
    winner: string
}

function sendGameState() {
    useVarStore().connectedPlayers.forEach((player: Player) => {
        createMessage(useVarStore().username, MessageType.GameStateUpdate, JSON.stringify({
            players: players,
            board: board.value,
            winner: checkForWin()
        } as GameState)).then((message: Message) => {
            sendMessage(message, player.ip)
        })
    })
}

function onTurnAttempt(message: Message) {
    let parsedMessage = JSON.parse(message.content)
    let position: number[] = parsedMessage.position

    if (players[0].username === message.username && board.value[position[0]][position[1]][position[2]] === " ") {
        board.value[position[0]][position[1]][position[2]] = parsedMessage.icon as string
        players = cycleArray(players)
        sendGameState()
    }
}

function checkForWin(): string {

    return ""
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