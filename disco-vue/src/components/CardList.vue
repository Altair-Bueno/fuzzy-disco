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
  data() {
    return {
      cardlist: []
    }
  },
  methods: {
    async getIndividualPost(id) {
      let response = await fetch(`/api/posts/${id}`);
      if(response.ok) {
        let server_payload = response.json();
        this.cardlist.push(server_payload);
      }
    }
  },
  async beforeRouteEnter(to, from, next) {
    next(vm => vm.getIndividualPost("614091f5f79c92b0de5ec555"));
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