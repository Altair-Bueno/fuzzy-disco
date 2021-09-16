<template>
  <div>
    <div class="profile">
      <Navbar></Navbar>
      <ProfileHeader :alias="alias" :description="description" :avatar="avatar" :email="email"></ProfileHeader>
    </div>
   </div>

</template>

<script>

import Navbar from "@/components/Navbar";
import ProfileHeader from "@/components/user/ProfileHeader";

export default {
  name: "UserProfile",
  components: {Navbar, ProfileHeader},
  data() {
    return {
      alias: "",
      email: "",
      creation_date: Date,
      description: "Sample Description",
      avatar: ""
    }
  },
  async beforeRouteEnter(to, from, next) {
    const user = to.params.user;
    let response = await fetch(`/api/users/${user}`, {
      method: 'GET',
    });
    if(response.status >= 200 && response.status <= 299) {
      next();
    } else {
      next({name: from.name});
    }
  },
  methods: {
    async loadUserData() {
      await this.isAuthenticated();
      let response = await fetch(`/api/users`, {
        method: 'GET',
        headers: {
          'Authorization': 'Bearer' + this.getCookieValue(this.findCookie("access_token"))
        }
      });
      let server_payload = await response.json();
      if(response.ok) {
        this.description = server_payload["description"];
        this.email = server_payload["email"];
        this.creation_date = server_payload["creation_date"];
        if(server_payload["avatar"]) {
          this.avatar = server_payload["avatar"];
        } else {
          this.avatar = require("/src/assets/sample-user.png");
        }
      }
    },
    async isAuthenticated() {
      let res = false;
      let refreshToken = this.findCookie("refresh_token");
      if(refreshToken) {
        let accessToken = this.findCookie("access_token");
        if(!accessToken) {
          let payload = {
            refresh_token: this.getCookieValue(refreshToken)
          }
          //Fixme: Localhost
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
    },
    findCookie(name) {
      return document.cookie.split('; ').find(row => row.startsWith(`${name}=`));
    },
    getCookieValue(cookie) {
      return cookie.split("=")[1];
    },
  },
  mounted() {
    this.alias = this.$route.params.user;
    this.loadUserData(this.alias);
  }
}
</script>

<style scoped>
  .profile {
    display: flex;
    flex-wrap: wrap;
    align-content: flex-start;
    align-items: baseline;
  }
</style>