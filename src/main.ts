import { Ref, createApp, ref } from "vue";
import "./styles.css";
import App from "./App.vue";
import router from "./router";
import { createPinia, defineStore } from 'pinia'
import { Message, MessageType } from "./components/Message";
import { invoke } from "@tauri-apps/api";

export type Player = {
  username: string
  ip: string
  icon: string
}

export type PlayerInfo = {
  username: string
  icon: string
}

const pinia = createPinia();
createApp(App).use(router).use(pinia).mount("#app");

export const useCallbackStore = defineStore('callback', {
  state: () => {
    return {functions: new Map<MessageType, ((message: Message) => void)[]>()  }
  },
  actions: {
    register(type: MessageType, fun: ((message: Message) => void)) {
      if (this.functions.get(type) == undefined) {
        this.functions.set(type, [])
      }
      this.functions.get(type)?.push(fun)
    },
    deregister(type: MessageType, fun: ((message: Message) => void)) {
      //if fun is registered for the messagetype
      if (this.functions.get(type)?.indexOf(fun) != -1) {
        let updatedList = this.functions.get(type)?.filter( f => f != fun)
        if (updatedList != undefined) {
          this.functions.set(type, updatedList)
        }
      }
    },
    clear() {
      this.functions.clear()
    }
  },
});

export const useVarStore = defineStore('var', {
  state: () => {                      //DOES NOT INCLUDE HOST
    return {username: "", hostIp: "", connectedPlayers: ref([]) as Ref<Player[]>, icon: ""}
  }
});

export function sendMessage(message: Message, encryptedIp: string) {
  let messageString = JSON.stringify(message);
  invoke('send_message_encrypted_ip', {message: messageString, encryptedIp: encryptedIp});
}