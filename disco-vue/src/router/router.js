import Vue from 'vue'
import VueRouter from 'vue-router'
import LoginPage from "@/components/auth-components/LoginPage";
import RegisterPage from "@/components/auth-components/RegisterPage";
import CardList from "@/components/CardList";
import Navbar from "@/components/Navbar";

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'home',
    component: {
      default: CardList,
      navbar: Navbar
    },
    meta: {
      requiresAuth: true
    }
  },
  {
    path: '/login',
    name: 'login',
    component: LoginPage
  },
  {
    path: '/signup',
    name: 'register',
    component: RegisterPage
  }
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes,
})

router.beforeEach((to, from, next) => {
  let isAuthenticated = document.cookie.split('; ').find(row => row.startsWith('access_token='))
  if(to.name !== 'login' && to.name !== 'register' && !isAuthenticated) {
    next({name: 'login'});
  } else {
    next();
  }
})

export default router
