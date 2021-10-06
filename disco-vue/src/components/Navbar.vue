<template>
  <div>
    <nav class="navbar">
      <ul class="navbar-nav">
        <li class="nav-item">
          <a class="nav-link" href="/">
            <img class="nav-logo" src="../assets/logo.png">
          </a>
        </li>
        <li class="nav-item">
          <RouterLink to="/home" class="nav-link">
            Home
          </RouterLink>
        </li>
        <li class="nav-item">
          <RouterLink to="/new-post" class="nav-link">
            New Post
          </RouterLink>
        </li>
        <li class="nav-item">
          <div class="search-cont">
            <input class="search-box" placeholder="Search">
          </div>
        </li>
        <li class="nav-item">
          <button class="drop-btn">User</button>
          <div class="drop-content">
            <RouterLink :to="aliasRoute">Profile</RouterLink>
            <RouterLink to="/user/sample-user/settings">Settings</RouterLink>
          </div>
        </li>
      </ul>
    </nav>
  </div>
</template>

<script>
export default {
  name: "Navbar",
  data() {
    return {
      aliasRoute: ""
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
      if(response.ok) {
        let server_payload = await response.json();
        this.aliasRoute = "/user/" + server_payload["alias"] + "/profile";
        console.log(this.aliasRoute)
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
    this.loadUserData();
  }
}
</script>

<style scoped>
  * {
    --navbar-color: rgb(40, 42, 53);
    --navbar-hover: rgb(30, 32, 53)
  }
  .navbar {
    position: relative;
    display: flex;
    height: 4rem;
    width: 100vw;
    background-color: var(--navbar-color);
    color: whitesmoke;
    font-family: "Open Sans", sans-serif;
  }

  .navbar-nav {
    display: flex;
    align-items: center;
    margin: 0;
    padding: 0;
    width: 100%;
    list-style: none;
  }

  .nav-link {
    display: flex;
    color: whitesmoke;
    align-items: center;
    justify-content: center;
    height: 4rem;
    width: 6rem;
    text-decoration: none;
    transition: 250ms;
  }

  .nav-link:hover {
    background-color: var(--navbar-hover);
    cursor: pointer;
    opacity: 0.7;
    color: rgba(0, 250, 154, 1);
    box-shadow: rgba(0, 250, 154, 0.66) 0 0 10px 0;
  }

  .search-cont {
    position: relative;
  }

  .search-box {
    margin: 0 1.5rem;
    border: none;
    border-bottom: 1px solid #ccc;
    height: 1.5rem;
    width: 5rem;
    background-color: var(--navbar-color);
    color: whitesmoke;
    transition: 300ms;
    font-family: "Open Sans", sans-serif;
    font-size: 16px;
    opacity: 0.9;
    font-weight: lighter;
  }

  .search-box:focus {
    outline: none;
    width: 15rem;
    border-color: rgba(0, 250, 154, 0.66);
  }

  .search-box::placeholder {
    opacity: 0.3;
    font-weight: lighter;
    font-size: 14px;
    font-family: "Open Sans", sans-serif;
  }

  .nav-logo {
    font-size: x-large;
    width: 2rem;
    margin: 0 1.5rem;
  }

  .nav-item:last-child {
    margin-left: auto;
  }

  .drop-btn {
    background-color: var(--navbar-color);
    color: white;
    padding: 16px;
    font-size: 16px;
    border: none;
    cursor: pointer;
    height: 4rem;
    width: 10rem;
    transition: 250ms;
  }

  .drop-content {
    display: none;
    position: absolute;
    background-color: rgb(255,250,240);
    min-width: 10rem;
    box-shadow: 0 8px 16px 0 rgba(0,0,0,0.2);
    z-index: 1;
    right: 0;
    transition: 250ms;
  }

  .drop-content a {
    color: black;
    padding: 12px 16px;
    text-decoration: none;
    display: block;
  }

  .drop-content a:hover {
    background-color: rgb(245,240,230)
  }

  .nav-item:hover .drop-content {
    display: block;
  }

  .nav-item:hover .drop-btn {
    background-color: var(--navbar-hover);
    cursor: pointer;
    opacity: 0.7;
    color: rgba(0, 250, 154, 1);
    box-shadow: rgba(0, 250, 154, 0.66) 0 0 10px 0;
  }

</style>