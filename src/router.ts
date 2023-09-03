import { createRouter, createWebHistory } from 'vue-router'
import Network from './components/Network.vue'
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
        path: '/Network',
        component: Network,
    },
    {
        path: '/Test',
        component: Test,
    }
  ]
})