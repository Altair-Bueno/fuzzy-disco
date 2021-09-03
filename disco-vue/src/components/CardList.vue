<template>
  <div>
    <div class="card-list" :key="card.id" v-for="card in cardlist">
      <CardItem v-bind:card="card"></CardItem>
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
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }
</style>