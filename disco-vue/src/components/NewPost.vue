<template>
  <div class="general">
    <Navbar></Navbar>
    <div class="card-edit">
      <h1>Drop or select both image and audio files</h1>
      <h1>Write a title for your card!</h1>
      <br>
      <br>
      <div class="form-cont">
        <div class="card">
          <div class="card-bg" id="drop" @dragover.prevent @drop.stop.prevent="processFile"></div>
          <h1 class="card-title">{{ title }}</h1>
          <PlayComp :audio="audio_url"></PlayComp>
        </div>
        <div class="form">
          <FormInput @input-update="updateTitle" field="Title" input-type="text" :input-ok="titleOk" identifier="title"></FormInput>
          <textarea v-model="caption" :class="[caption ? 'caption-box-open' : 'caption-box', {'invalid-input': !captionOk}]"></textarea>
          <br>
          <div class="radio">
            <div>
              <input type="radio" name="visibility" id="public"
                     :checked="isPublic" v-on:input="visibility = 'Public'">
              <label for="public">Public</label>
            </div>
            <div>
              <input type="radio" name="visibility" id="private"
                     :checked="isPrivate" v-on:input="visibility = 'Private'">
              <label for="private">Private</label>
            </div>
          </div>
          <br>
          <button @click="uploadCard" class="submit-btn">Upload Card</button>
        </div>
      </div>
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
      visibility: "Public",
      audio_url: "",

      titleOk: true,
      captionOk: true,
      visibilityOk: true,
      imageOk: true,
      audioOk: true,

      audioKey: "",
      imageKey: ""
    }
  },
  computed: {
    isPublic() {
      return this.visibility === 'Public';
    },
    isPrivate() {
      return this.visibility === 'Private';
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
      if(!this.validatePost()) {
        console.log("Error validating posts");
      } else {
        await this.isAuthenticated();
        this.audioKey = await this.uploadMedia(this.audio);
        this.imageKey = await this.uploadMedia(this.image);
        let payload = {
          title: this.title,
          caption: this.caption,
          audio: this.audioKey,
          photo: this.imageKey,
          visibility: this.visibility,
        };
        let response = await fetch("/api/posts/new", {
          method: 'POST',
          headers: {
            'Authorization': "Bearer " + this.getCookieValue(this.findCookie("access_token")),
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(payload)
        });
        //TODO Redirect to Card Page
        if(response.ok) {
          await this.$router.push({name: 'home'})
        }
      }
    },
    async uploadMedia(media) {
      let response = await fetch("/api/media/upload", {
        method: 'POST',
        headers: {
          'Authorization': "Bearer " + this.getCookieValue(this.findCookie("access_token")),
        },
        body: media
      });
      let res;
      if(response.ok) {
        let server_payload = await response.json();
        res = server_payload.key;
      }
      return res;
    },
    async isAuthenticated() {
      let res = false;
      let refreshToken = this.findCookie("refresh_token");
      if(refreshToken) {
        let accessToken = this.findCookie("access_token");
        if(!accessToken) {
          let payload = {
            refresh_token: this.getCookieValue(refreshToken)
          }
          //Fixme: Localhost
          let response = await fetch("/api/users/auth/login?using=refresh_token", {
            method: "POST",
            headers: {
              'Content-Type': 'application/json'
            },
            body: JSON.stringify(payload)
          });
          let server_payload = await response.json();
          console.log(server_payload);
          let status_code = response.status;
          if(status_code >= 200 && status_code <= 299) {
            let ttl = server_payload.expires_in * 1000;
            console.log(ttl);
            let a = "access_token=" + server_payload.access_token + "; SameSite=Lax; expires=" + (new Date(Date.now() + ttl)).toUTCString() + ";";
            document.cookie = a;
            console.log(a);
            console.log(document.cookie);

            res = true;
          } else {
            alert(status_code + " error");
          }
        } else {
          res = true;
        }
      }
      return res;
    },
    findCookie(name) {
      return document.cookie.split('; ').find(row => row.startsWith(`${name}=`));
    },
    getCookieValue(cookie) {
      return cookie.split("=")[1];
    },
    validatePost() {
      this.titleOk = this.validateTitle(this.title);
      this.captionOk = this.validateCaption(this.caption);
      this.visibilityOk = this.validateVisibility(this.visibility);
      this.imageOk = this.validateImage(this.image);
      this.audioOk = this.validateAudio(this.audio);
      return (this.titleOk && this.captionOk && this.visibilityOk && this.imageOk && this.audioOk);
    },

    validateTitle(title) {
      let val = true;
      const regex = /^(\S+.*\S)|\S$/;
      if(!(regex.test(title) && title.length <= 24)) {
        val = false;
        //alert("Title must have between 1 and 24 characters and no blank spaces at the beginning or end");
      }
      return val;
    },
    validateCaption(caption) {
      let val = true;
      if(!(caption.length <= 150)) {
        val = false;
        //alert("Caption must have less than 150 characters");
      }
      return val;
    },
    validateVisibility(visibility) {
      let val = true;
      if(!(visibility === 'Public' || visibility === 'Private')) {
        val = false;
        //alert("Post visibility must be chosen");
      }
      return val;
    },
    validateImage(image) {
      let val = true;
      if(!image.size) {
        val = false;
        //alert("Invalid or missing image file");
      }
      return val;
    },
    validateAudio(audio) {
      let val = true;
      if(!audio.size) {
        val = false;
        //alert("Invalid or missing audio file");
      }
      return val;
    },
    updateTitle(title) {
      this.title = title.update;
    }
  }
}
</script>

<style scoped>
  .general {
    font-family: "Open Sans", sans-serif;
  }
  .form-cont {
    display: flex;
  }
  .card-edit {
    display: flex;
    flex-direction: column;
    align-items: center;
    color: whitesmoke;
  }
  .card {
    margin: 25px;
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

  .form {
    margin: 25px;
    display: flex;
    flex-direction: column;
    align-items: center;
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
    height: 7rem;
  }
  .caption-box-open {
    resize: none;
    height: 7rem;
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

  .invalid-input {
    outline: none;
    border-color: red;
  }
</style>