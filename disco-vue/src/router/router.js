import Vue from 'vue'
import VueRouter from 'vue-router'
import LoginPage from "@/components/auth-components/LoginPage";
import RegisterPage from "@/components/auth-components/RegisterPage";
import CardList from "@/components/CardList";
import UserProfile from "@/components/profile-components/UserProfile";

Vue.use(VueRouter)

const routes = [
    {
        path: '/',
        redirect: '/home',
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
    },
    {
        path: '/home',
        name: 'home',
        component: CardList,
        meta: {
            requiresAuth: true
        }
    },
    {
        path: '/user/:user',
        name: 'user',
        component: UserProfile,
    }
]

const router = new VueRouter({
    mode: 'history',
    base: process.env.BASE_URL,
    routes,
})

router.beforeEach(async (to, from, next) => {
    if(to.name !== 'login' && to.name !== 'register' && !(await isAuthenticated())) {
        next({name: 'login'});
    } else {
        next();
    }
})

async function isAuthenticated() {
    let res = false;
    let refreshToken = findCookie("refresh_token");
    if(refreshToken) {
        let accessToken = findCookie("access_token");
        if(!accessToken) {
            let payload = {
                refresh_token: getCookieValue(refreshToken)
            }
            let response = await fetch("/api/users/auth/login?using=refresh_token", {
                method: "POST",
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(payload)
            });
            let server_payload = await response.json();
            console.log(server_payload);
            let status_code = response.status;
            if(status_code >= 200 && status_code <= 299) {
                let ttl = server_payload.expires_in * 1000;
                console.log(ttl);
                let a = "access_token=" + server_payload.access_token + "; SameSite=Lax; expires=" + (new Date(Date.now() + ttl)).toUTCString() + ";";
                document.cookie = a;
                console.log(a);
                console.log(document.cookie);

                res = true;
            } else {
                alert(status_code + " error");
            }
        } else {
            res = true;
        }
    }
    return res;
}

function findCookie(name) {
    return document.cookie.split('; ').find(row => row.startsWith(`${name}=`));
}
function getCookieValue(cookie) {
    return cookie.split("=")[1];
}



export default router
