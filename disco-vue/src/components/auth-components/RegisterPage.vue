<template>
  <div class="object-cont">
    <div class="form-cont">
      <form class="login-form">
        <h1>Sign Up</h1>
        <FormInput @input-update="getEmail" identifier="email" field="Email" :input-ok="emailOk"></FormInput>
        <FormInput @input-update="getUsername" inputType="email" identifier="alias" field="Username" :input-ok="usernameOk"></FormInput>
        <FormInput @input-update="getPasswd" inputType="password" identifier="pwd" field="Password" :input-ok="passwdOk"></FormInput>
        <FormInput @input-update="getRepeatPasswd" inputType="password" identifier="repeat-pwd" field="Repeat Password" :input-ok="repeatPasswdOk"></FormInput>
      </form>
      <button @click="submit" class="submit-btn">Register</button>
      <div class="login-text">
        <p>Already have an account? <a class="login-link" href="#">Login here</a></p>
      </div>
    </div>
  </div>
</template>

<script>
import FormInput from "@/components/auth-components/FormInput";
export default {
  name: "RegisterPage",
  components: {FormInput},
  data() {
    return {
      email: String,
      username: String,
      passwd: String,
      repeatPasswd: String,

      emailOk: true,
      usernameOk: true,
      passwdOk: true,
      repeatPasswdOk: true
    }
  },
  methods: {
    async submit() {
      if(!this.validateUser()) {
        console.log("repeat");
      } else {
        let user = {
          alias: this.username,
          email: this.email,
          password: this.passwd
        }
        let response = await fetch('api/users/auth/signup', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(user)
        });
        let result = await response.json();
        console.log(result.message);
      }
    },

    validateUser() {
      if(!this.validateEmail(this.email)) {
        this.emailOk = false;
        return false;
      }
      if(!this.validateUsername(this.username)) {
        this.usernameOk = false;
        return false;
      }
      if(!this.validatePasswd(this.passwd)) {
        this.passwdOk = false;
        return false;
      }
      if(!this.validateRepeatPasswd(this.repeatPasswd)) {
        this.repeatPasswdOk = false;
        return false;
      }
      this.emailOk = true;
      this.usernameOk = true;
      this.passwdOk = true;
      this.repeatPasswdOk = true;
      return true;
    },

    getEmail(email) {
      this.email = email.update;
    },
    getUsername(username) {
      this.username = username.update;
    },
    getPasswd(passwd) {
      this.passwd = passwd.update;
    },
    getRepeatPasswd(repeatPasswd) {
      this.repeatPasswd = repeatPasswd.update;
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
    validateRepeatPasswd(repeatPasswd) {
      return repeatPasswd === this.passwd;
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
    height: 30rem;
    width: 20rem;
    box-shadow: 5px 5px 25px 10px rgba(0, 0, 0, 0.5);
  }

  .submit-btn {
    font-family: "Open Sans", sans-serif;
    color: #444444;
    font-weight: bold;
    font-size: 1rem;
    border: none;
    height: 2rem;
    width: 9rem;
    cursor: pointer;
    background-color: whitesmoke;
    border-radius: 25px;
    transition: 300ms;
  }

  .submit-btn:hover {
    background-color: var(--login-border);
    width: 15rem;
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