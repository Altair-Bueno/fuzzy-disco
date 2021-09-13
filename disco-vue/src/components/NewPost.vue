<template>
  <div class="general">
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
      <FormInput field="Title" input-type="text" :input-ok=true identifier="title"></FormInput>
      <textarea v-model="caption" :class="[caption ? 'caption-box-open' : 'caption-box']"></textarea>
      <br>
      <div class="radio">
        <div>
          <input type="radio" name="visibility" value="public" id="public"
                 v-model="visibility">
          <label for="public">Public</label>
        </div>
        <div>
          <input type="radio" name="visibility" value="private" id="private"
                 v-model="visibility">
          <label for="private">Private</label>
        </div>
      </div>
      <br>
      <button @click="uploadCard" class="submit-btn">Upload Card</button>
    </div>
  </div>
</template>

<script>
import Navbar from "@/components/Navbar";
import PlayComp from "@/components/card/PlayComp";
import FormInput from "@/components/auth/FormInput";

export default {
  name: "NewPost",
  components: {FormInput, PlayComp, Navbar},
  data() {
    return {
      title: "",
      caption: "",
      image: File,
      audio: File,
      visibility: "",
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
    async uploadCard() {
      console.log(this.visibility);
    }
  }
}
</script>

<style scoped>
  .general {
    font-family: "Open Sans", sans-serif;
  }
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

  .caption-box {
    resize: none;
    height: 1.5rem;
    width: 20rem;
    outline: none;
    overflow: auto;
    font-family: "Open Sans", sans-serif;
    font-size: 16px;
    border-radius: 10px;
    padding: 10px;
    transition: 300ms;
  }
  .caption-box:focus {
    height: 10rem;
  }
  .caption-box-open {
    resize: none;
    height: 10rem;
    width: 20rem;
    outline: none;
    overflow: auto;
    font-family: "Open Sans", sans-serif;
    font-size: 16px;
    border-radius: 10px;
    padding: 10px;
    transition: 300ms;
  }

  ::-webkit-scrollbar {
    width: 0;
  }

  .radio {
    width: 10rem;
    display: flex;
    justify-content: space-between;
  }

  .submit-btn {
    font-family: "Open Sans", sans-serif;
    color: #444444;
    font-weight: bold;
    font-size: 1rem;
    border: none;
    width: 7rem;
    height: 2rem;
    cursor: pointer;
    background-color: whitesmoke;
    border-radius: 25px;
    transition: 300ms;
  }

  .submit-btn:hover {
    background-color: rgba(0, 250, 154, 1);
    width: 15rem;
  }
</style>