<template>
  <div>
    <Navbar></Navbar>
    <div class="card-edit">
      <h1>Drop or select both image and audio files</h1>
      <h1>Write a title for your card!</h1>
      <br>
      <br>
      <div class="card">
        <div class="card-bg" id="drop" @dragover.prevent @drop.stop.prevent="processFile"></div>
        <h1 class="card-title">{{ title }}</h1>
        <PlayComp :audio="audio_url"></PlayComp>
      </div>
      <br>
      <input v-model="title" class="input-title" type="text" placeholder="Title here">
    </div>

  </div>
</template>

<script>
import Navbar from "@/components/Navbar";
import PlayComp from "@/components/card/PlayComp";

export default {
  name: "NewPost",
  components: {PlayComp, Navbar},
  data() {
    return {
      title: "",
      image: File,
      audio: File,
      audio_url: ""
    }
  },
  methods: {
    async processFile(event) {
      const file = event.dataTransfer.files[0];
      if(file.type.startsWith('image/')) {
        this.image = file;
        const img = document.getElementById("drop");
        img.file = file;
        const reader = new FileReader();
        reader.onload = ((img) => {
          return (e) => {
            img.style.backgroundImage = "url(" + e.target.result + ")";
          };
        }) (img);
        reader.readAsDataURL(file);
      } else if(file.type.startsWith('audio/')) {
        this.audio = file;
        let audio_url = "";
        const reader = new FileReader();
        // eslint-disable-next-line no-unused-vars
        reader.onload = ((audio) => {
          return (e) => {
            this.audio_url = e.target.result;
          }
        }) (audio_url);
        reader.readAsDataURL(file);
      }
    },
  }
}
</script>

<style scoped>
  .card-edit {
    display: flex;
    flex-direction: column;
    align-items: center;
    color: whitesmoke;
  }
  .card {
    position: relative;
    border-radius: 35px;
    height: 320px;
    width: 240px;
    overflow: hidden;
    box-shadow: 0 0 3px 0 #ddd;
    transition: 350ms;
  }
  .card-bg {
    position: absolute;
    opacity: 0.5;
    height: 100%;
    width: 100%;
    top: -20px;
    left: -20px;
    padding: 20px;
    background-repeat: no-repeat;
    background-position: center;
    background-size: cover;
    transition: 600ms;
  }
  .card-title {
    text-align: center;
    opacity: 1;
    font-size: 26px;
    position: relative;
    margin: 10px;
    color: #eeeeee;
    cursor: default;
    transition: 200ms;
    border: none;
    background: none;
  }
  .card-title:focus {
    opacity: 1;
    position: relative;
    margin: 10px;
    color: #eeeeee;
    cursor: default;
    transition: 200ms;
    border: none;
    background: none;
    outline: none;
  }
  .input-title {
    margin: 0 1.5rem;
    border: none;
    border-bottom: 1px solid #ccc;
    height: 1.5rem;
    width: 5rem;
    background-color: var(--navbar-color);
    color: whitesmoke;
    transition: 300ms;
    font-family: "Open Sans", sans-serif;
    font-size: 16px;
    opacity: 0.9;
    font-weight: lighter;
  }

  .input-title:hover {
    outline: none;
    width: 15rem;
    border-color: var(--login-border);
  }

  .input-title:focus {
    outline: none;
    width: 15rem;
    border-color: var(--login-border);
  }
</style>