<template>
  <div class="object-cont">
    <div class="form-cont">
      <form class="login-form">
        <h1>Login</h1>
        <FormInput @input-update="getEmailUsername" identifier="email" field="Email or Username" :input-ok="(emailOk || usernameOk)"></FormInput>
        <FormInput @input-update="getPasswd" inputType="password" identifier="pwd" field="Password" :input-ok="passwdOk"></FormInput>
      </form>
      <button @click="submit" class="submit-btn">Login</button>
      <br>
      <div class="login-text">
        <p>Dont have an account yet? <RouterLink class="login-link" to="/signup">Register now</RouterLink></p>
      </div>
    </div>
  </div>
</template>

<script>
import FormInput from "@/components/auth-components/FormInput";

export default {
  name: "LoginPage",
  components: {FormInput},
  data() {
    return {
      emailUsername: String,
      passwd: String,

      emailOk: true,
      usernameOk: true,
      passwdOk: true
    }
  },
  methods: {
    async submit() {
      let loginMethod = this.validateUser();
      if(loginMethod === "") {
        console.log("repeat");
      } else {

        let user = {
          [loginMethod]: this.emailUsername,
          password: this.passwd
        }
        let response = await fetch(`/api/users/auth/login?using=${loginMethod}`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(user)
        });

        let server_payload = await response.json();
        let status_code = response.status;
        if(status_code >= 400 && status_code <= 499) {
          this.emailOk = false;
          this.usernameOk = false;
          this.passwdOk = false;
          alert(`${loginMethod} or password incorrect`);

        } else if(status_code >= 200 && status_code <= 299) {
          let ttl = server_payload.expires_in * 1000;
          document.cookie = "refresh_token=" + server_payload.refresh_token + "; SameSite=Lax; expires=" + new Date(9999, 1, 1) + ";";
          document.cookie = "access_token=" + server_payload.access_token + "; SameSite=Lax; expires=" + (new Date(Date.now() + ttl)).toUTCString() + ";";
          await this.$router.push({name: 'home'});

        } else {
          alert("Server error. Try later.")
        }
      }
    },

    validateUser() {
      let loginMethod = this.validateEmailUsername(this.emailUsername);
      if(loginMethod === "email") {
        this.emailOk = true;
      } else if(loginMethod === "alias") {
        this.usernameOk = true;
      } else {
        this.emailOk = false;
        this.usernameOk = false;
      }
      this.passwdOk = this.validatePasswd(this.passwd);
      return loginMethod;
    },

    getEmailUsername(emailUsername) {
      this.emailUsername = emailUsername.update;
    },
    getPasswd(passwd) {
      this.passwd = passwd.update;
    },

    validateEmail(email) {
      const regex = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
      return regex.test(email);
    },
    validateUsername(username) {
      const regex =/^[a-zA-Z_\-0-9]{4,30}$/;
      return regex.test(username);
    },
    validatePasswd(passwd) {
      return passwd.length >= 8;
    },
    validateEmailUsername(emailUsername) {
      let res = "";
      if(this.validateEmail(emailUsername)) {
        res = "email"
      } else if(this.validateUsername(emailUsername)) {
        res = "alias";
      }
      return res;
    }
  }

}
</script>

<style scoped>
  * {
    --login-border: rgba(0, 250, 154, 0.66)
  }

  .object-cont {
    font-family: "Open Sans", sans-serif;
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 5rem;
  }

  .form-cont {
    position: relative;
    border-radius: 25px;
    color: whitesmoke;
    border: 1px solid var(--login-border);
    padding: 5rem;
    height: 25rem;
    width: 20rem;
    box-shadow: 5px 5px 25px 10px rgba(0, 0, 0, 0.5);
  }

  .submit-btn {
    font-family: "Open Sans", sans-serif;
    color: #444444;
    font-weight: bold;
    font-size: 1rem;
    border: none;
    width: 5rem;
    height: 2rem;
    cursor: pointer;
    background-color: whitesmoke;
    border-radius: 25px;
    transition: 300ms;
  }

  .submit-btn:hover {
    background-color: var(--login-border);
    width: 10rem;
  }

  h1:hover {
    cursor: default;
  }

  .login-text {
    margin-top: 6rem;
  }

  .login-link {
    color: var(--login-border);
  }

</style>