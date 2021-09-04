<template>
  <div>
    <div class="card-list">
      <CardItem v-bind:card="card" :key="card.id" v-for="card in cardlist"></CardItem>
    </div>
  </div>
</template>

<script>
import CardItem from "@/components/CardItem";
export default {
  name: "CardList",
  components: {CardItem},
  props: ['cardlist'],
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