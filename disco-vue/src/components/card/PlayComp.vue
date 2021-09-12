<template>
  <div @click="playAudio" class="card-play">
    <i class="far fa-circle"></i>
    <i class="fas fa-play"></i>
  </div>
</template>

<script>
export default {
  name: "PlayComp",
  props: ['audio'],
  data() {
    return {
      isPlaying: false,
      isCreated: false,
      audio_player: new Audio()
    }
  },
  methods: {
    playAudio() {
      if(this.isCreated) {
        if(this.isPlaying) {
          this.audio_player.pause();
          this.isPlaying = false;
        } else {
          this.audio_player.loop;
          this.audio_player.play();
          this.isPlaying = true;
        }
      } else {
        if(this.audio) {
          this.audio_player = new Audio(this.audio);
          this.audio_player.volume = 0.5;
          this.audio_player.play();
          this.isCreated = true;
          this.isPlaying = true;
        }
      }
    }
  }
}
</script>

<style scoped>
  .card-play {
    position: absolute;
    top: -15%;
    left: 50%;
    display: flex;
    justify-content: center;
    align-items: center;
    color: whitesmoke;
    cursor: pointer;
    transition: 350ms;
  }

  .fa-circle {
    position: absolute;
    font-size: 80px;
    border-radius: 50%;
    box-shadow: rgba(0, 0, 0, 0.66) 0 30px 60px 0, inset #ccc 0 0 0 5px;
    animation: play-reverse 200ms ease-out;
    animation-fill-mode: forwards;
  }

  .fa-play {
    position: absolute;
    font-size: 20px;
    box-shadow: rgba(0, 0, 0, 0.66) 0 30px 60px 0;
    animation: play-reverse 300ms ease-out;
    animation-fill-mode: forwards;
  }

  .card:hover .fa-circle {
    animation: play 200ms ease-out;
    animation-fill-mode: forwards;
  }
  .card:hover .fa-play {
    animation: play 350ms ease-out;
    animation-fill-mode: forwards;
  }
  @keyframes play {
    0% {
      transform: translate(0, 0);
    }
    100% {
      transform: translate(0, 175px);
    }
  }
  @keyframes play-reverse {
    0% {
      transform: translate(0, 175px);
    }
    100% {
      transform: translate(0, 0);
      display: none;
    }
  }
</style>