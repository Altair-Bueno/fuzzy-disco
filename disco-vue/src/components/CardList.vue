<template>
  <div>
    <div class="card-list">
      <Navbar></Navbar>
      <CardItem v-bind:card="card" :key="card.id" v-for="card in cardlist"></CardItem>
    </div>
  </div>
</template>

<script>
import CardItem from "@/components/card-components/CardItem";
import Navbar from "@/components/Navbar";
export default {
  name: "CardList",
  components: {Navbar, CardItem},
  data() {
    return {
      cardlist: [{
        id: 0,
        title: "Killua and Gon",
        caption: "Daichi wo fumishimete kimi wa mezameteiku Tenshi no hohoemi de tsuredashite!",
        author_id: "Carlos",
        photo_path: require("../assets/sample-image.png"),
        audio_path: ""
      },
      {
        id: 1,
        title: "Only Killua",
        caption: "aaaaaaaaaaa sisisisisisiis siisi sisi",
        author_id: "Altair",
        photo_path: require("../assets/sample-image-2.png"),
        audio_path: ""
      },
      {
        id: 2,
        title: "Wing te amo",
        caption: "te amo",
        author_id: "Pito",
        photo_path: require("../assets/sample-image-3.jpg"),
        audio_path: ""
      },
      {
        id: 3,
        title: "Killua and Gon",
        caption: "Daichi wo fumishimete kimi wa mezameteiku Tenshi no hohoemi de tsuredashite!",
        author_id: "Carlos",
        photo_path: require("../assets/sample-image.png"),
        audio_path: ""
      },
      {
        id: 4,
        title: "Only Killua",
        caption: "aaaaaaaaaaa sisisisisisiis siisi sisi",
        author_id: "Altair",
        photo_path: require("../assets/sample-image-2.png"),
        audio_path: ""
      },
      {
        id: 5,
        title: "Wing te amo",
        caption: "te amo",
        author_id: "Pito",
        photo_path: require("../assets/sample-image-3.jpg"),
        audio_path: ""
      }]
    }
  }
}
</script>

<style scoped>
  .card-list {
    display: flex;
    flex-wrap: wrap;
    align-content: flex-start;
    align-items: baseline;
    justify-content: space-around;
  }
</style>