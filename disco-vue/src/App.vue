<template>
  <div id="app">
    <Navbar></Navbar>
    <CardList v-bind:cardlist="cards"></CardList>
  </div>
</template>

<script>

import CardList from "@/components/CardList";
import Navbar from "@/components/Navbar";

export default {
  name: 'App',
  components: {
    Navbar,
    CardList,
  },
  data() {
    return {
      cards: [{
        id: 0,
        title: "Killua and Gon",
        caption: "Daichi wo fumishimete kimi wa mezameteiku Tenshi no hohoemi de tsuredashite!",
        author_id: "Carlos",
        photo_path: require("./assets/sample-image.png"),
        audio_path: ""
      },
      {
        id: 1,
        title: "Only Killua",
        caption: "aaaaaaaaaaa sisisisisisiis siisi sisi",
        author_id: "Altair",
        photo_path: require("./assets/sample-image-2.png"),
        audio_path: ""
      },
      {
        id: 2,
        title: "Wing te amo",
        caption: "te amo",
        author_id: "Pito",
        photo_path: require("./assets/sample-image-3.jpg"),
        audio_path: ""
      }]
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
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  margin: 0;
  color: #2c3e50;
  display: block;
}
</style>
