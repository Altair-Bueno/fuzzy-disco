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
        <h1 class="card-title">Text</h1>
        <PlayComp></PlayComp>
      </div>
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
      image: File,
    }
  },
  methods: {
    processFile(event) {
      const file = event.dataTransfer.files[0];
      if(file.type.startsWith('image/')) {
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
        const reader = new FileReader();
        let audio_url;
        reader.onload = ((audio) => {
          return (e) => {
            audio = new Audio(e.target.result);
            audio.loop;
            audio.play();
          };
        }) (audio_url);
        reader.readAsDataURL(file);
      }
    }
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
    opacity: 1;
    position: relative;
    margin: 10px;
    color: #eeeeee;
    cursor: default;
    transition: 200ms;
  }
</style>