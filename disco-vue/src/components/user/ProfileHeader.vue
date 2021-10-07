<template>
  <div class="wrap">
    <div class="header">
      <img class="avatar" :src="avatar" alt="alt">
      <div class="text-box">
        <h1 class="alias">{{ alias }}</h1>
        <div v-show="showDesc">
          <p class="description">
            {{ description }}
          </p>
          <div @click="spawnEdit">
            <i class="far fa-edit edit"></i>
          </div>
        </div>
        <div v-show="!showDesc">
          <textarea class="edit-description"
                    v-model="description"
                    :class="[description ? 'edit-description-open' : 'edit-description']"></textarea>
          <div @click="spawnEdit">
            <i class="far fa-edit edit"></i>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>


export default {
  name: "ProfileHeader",
  components: {},
  props: {
    alias: String,
    description: String,
    avatar: String,
    email: String
  },
  data() {
    return {
      showDesc: true
    }
  },
  methods: {
    async spawnEdit() {
      if(!this.showDesc) {
        await this.changeDescription();
      }
      this.showDesc = !this.showDesc;
    },
    async changeDescription() {
      await this.isAuthenticated();
      let response = await fetch("/api/users/update", {
        method: 'POST',
        headers: {
          'Authorization': "Bearer " + this.getCookieValue(this.findCookie("access_token")),
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({'description': this.description})
      });
      if(response.ok) {
        alert("Description updated successfully");
      } else {
        alert("Error 500: Internal server error");
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
    }
  }
}
</script>

<style scoped>
  .wrap {
    display: flex;
    flex-direction: row;
    align-items: baseline;
    justify-content: flex-start;
    color: whitesmoke;
    font-family: "Open Sans", sans-serif;
  }
  .header {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 10px;
    margin: 10px;
    border: 1px solid #666666;
    border-radius: 25px;
  }

  .avatar {
    margin: 10px;
    height: 12rem;
    width: 12rem;
    border-radius: 50%;
  }

  .text-box {
    margin: 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .description {
    width: 12rem;
  }

  .edit {
    cursor: pointer;
    margin-left: 10px;
  }

  .edit-description {
    resize: none;
    height: 1.5rem;
    width: 10.5rem;
    outline: none;
    overflow: auto;
    font-family: "Open Sans", sans-serif;
    font-size: 16px;
    border-radius: 10px;
    padding: 10px;
    transition: 300ms;
  }
  .edit-description:focus {
    height: 7rem;
  }
  .edit-description-open {
    resize: none;
    height: 7rem;
    width: 10.5rem;
    outline: none;
    overflow: auto;
    font-family: "Open Sans", sans-serif;
    font-size: 16px;
    border-radius: 10px;
    padding: 10px;
    transition: 300ms;
  }

</style>