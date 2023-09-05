import { createRouter, createWebHistory } from 'vue-router'
import CreateLobby from './components/Menus/CreateLobby.vue'
import Test from './components/Test.vue'
import TitleScreen from './components/Menus/TitleScreen.vue'

export default createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: TitleScreen,
    },
    {
        path: '/CreateLobby',
        component: CreateLobby,
    },
    {
        path: '/Test',
        component: Test,
    }
  ]
})