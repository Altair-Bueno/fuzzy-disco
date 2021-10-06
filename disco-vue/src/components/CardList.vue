<template>
  <div>
    <div class="card-list">
      <CardItem v-bind:card="card" :key="card.id" v-for="card in cardlist"></CardItem>
    </div>
  </div>
</template>

<script>
import CardItem from "@/components/card/CardItem";

export default {
  name: "CardList",
  components: {CardItem},
  props: {
    queryURL: String,
  },
  data() {
    return {
      cardlist: [],
    }
  },
  methods: {
    async getXPosts() {
      let response = await fetch(this.queryURL +
          "block=0&date=" + encodeURIComponent((new Date()).toJSON()));
      if(response.ok) {
        let server_payload = await response.json();
        this.cardlist.push(...server_payload);
      }
    }
  },
  mounted() {
    this.getXPosts();
  },
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