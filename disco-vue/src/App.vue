<template>
  <div id="app">
    <CardItem></CardItem>
    <CardItem></CardItem>
    <CardItem></CardItem>
    <CardItem></CardItem>
  </div>
</template>

<script>

import CardItem from "@/components/Card";

export default {
  name: 'App',
  components: {
    CardItem,

  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
