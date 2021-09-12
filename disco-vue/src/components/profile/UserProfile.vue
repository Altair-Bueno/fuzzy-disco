<template>
  <div>
    <div class="profile">
      <Navbar></Navbar>
      <ProfileHeader :alias="alias" :description="description" :avatar="avatar"></ProfileHeader>
    </div>
   </div>
</template>

<script>

import Navbar from "@/components/Navbar";
import ProfileHeader from "@/components/profile/ProfileHeader";

export default {
  name: "UserProfile",
  components: {Navbar, ProfileHeader},
  data() {
    return {
      alias: String,
      email: String,
      creation_date: Date,
      description: "Sample description",
      avatar: require("/src/assets/logo.png")
    }
  },
  async beforeRouteEnter(to, from, next) {
    const user = to.params.user;
    let response = await fetch(`/api/users/${user}`, {
      method: 'GET',
    });
    if(response.status >= 200 && response.status <= 299) {
      next();
    } else {
      next({name: from.name});
    }
  },
  methods: {
    loadUserData: async function(user) {
      let response = await fetch(`/api/users/${user}`, {
        method: 'GET',
      });
      let status_code = response.status;
      let server_payload = await response.json();
      if(status_code <= 200 && status_code >= 299) {
        if(server_payload["description"]) {
          this.description = server_payload["description"];
        }
        if(server_payload["avatar"]) {
          this.avatar = server_payload["avatar"];
        }
      }
    }
  },
  mounted() {
    this.alias = this.$route.params.user;
    this.loadUserData(this.alias);
  }
}
</script>

<style scoped>
  .profile {
    display: flex;
    flex-wrap: wrap;
    align-content: flex-start;
    align-items: baseline;
    justify-content: space-around;
  }
</style>