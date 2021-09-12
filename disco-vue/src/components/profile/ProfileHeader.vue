<template>
  <div>
    <div class="header">
      <img class="avatar" :src="avatar" alt="alt">
      <div class="text-box">
        <h1 class="alias">{{ alias }}</h1>
        <p class="description">{{ description }}</p>
      </div>
    </div>
  </div>
</template>

<script>

export default {
  name: "ProfileHeader",
  props: {
    alias: String,
    description: String,
    avatar: String
  }
}
</script>

<style scoped>
  .header {
    display: flex;
    padding: 10px;
    margin: 10px;
    color: whitesmoke;
  }

  .avatar {
    margin: 10px;
  }

  .text-box {
    margin: 20px;
  }
</style>