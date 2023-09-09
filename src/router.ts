import { createRouter, createWebHistory } from 'vue-router'
import CreateLobby from './components/Menus/CreateLobby.vue'
import TitleScreen from './components/Menus/TitleScreen.vue'
import ChooseUsername from './components/Menus/ChooseUsername.vue'
import JoinLobby from './components/Menus/JoinLobby.vue'

export default createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: ChooseUsername,
    },
    {
        path: '/Title',
        component: TitleScreen
    },
    {
        path: '/CreateLobby',
        component: CreateLobby,
    },
    {
        path: '/JoinLobby',
        component: JoinLobby,
    }
  ]
})