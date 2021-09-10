<template>
  <div id="app">
    <RouterView name="navbar"></RouterView>
    <RouterView></RouterView>
  </div>
</template>

<script>

export default {
  name: 'App',
  components: {
  },
  data() {
    return {
    }
  },
  methods: {
    async requestData() {
      let cnt = 0;
      let response;
      do {
        response = await fetch("/api/post")
      } while(cnt < 100 || response);
    }
  }
}
</script>

<style>
  html {
    background-color: rgb(40, 42, 53);
  }
  #app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    margin: 0;
    color: #2c3e50;
    display: block;
  }

  #nav {
    padding: 30px;
  }

  #nav a {
    font-weight: bold;
    color: #2c3e50;
  }

  #nav a.router-link-exact-active {
    color: #42b983;
  }
</style>
