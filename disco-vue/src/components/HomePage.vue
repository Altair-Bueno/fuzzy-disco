<template>
  <div>
    <Navbar></Navbar>
    <CardList></CardList>
  </div>
</template>

<script>
import CardList from "@/components/CardList";
import Navbar from "@/components/Navbar";

export default {
  name: "HomePage",
  components: {Navbar, CardList}
}
</script>

<style scoped>

</style>