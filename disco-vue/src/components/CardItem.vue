<template>
  <div class="card-wrap">
    <div class="card">
      <div :style="cssVars" class="card-bg"></div>
      <h1 class="card-title">{{ card.title }}</h1>
      <PlayComp></PlayComp>
      <Slider :card="card" class="slider-comp"></Slider>
    </div>
  </div>
</template>

<script>
import Slider from "@/components/card-components/Slider";
import PlayComp from "@/components/card-components/PlayComp";

export default {
  name: "CardItem",
  components: {PlayComp, Slider},
  props: ['card'],
  computed: {
    cssVars() {
      return  {
        '--card-background': "url(" + this.card.photo_path + ")",
      }
    }
  }
}
</script>

<style scoped>
  .card-wrap {
    margin: 0;
    padding: 0;
    border: 0;
  }

  .card {
    position: relative;
    border-radius: 35px;
    height: 320px;
    width: 240px;
    overflow: hidden;
    outline: 1px solid rgba(3, 3, 3, 0.4);
    outline-offset: -10px;
    transition: 350ms;
  }

  .card-bg {
    --card-background: "";
    position: absolute;
    opacity: 0.5;
    height: 100%;
    width: 100%;
    top: -20px;
    left: -20px;
    padding: 20px;
    background-image: var(--card-background);
    background-repeat: no-repeat;
    background-position: center;
    background-size: cover;
    transition: 600ms;
  }

  .card-title {
    opacity: 1;
    position: relative;
    margin: 10px;
    color: #444444;
    cursor: default;
    transition: 200ms;
  }

  .slider-comp {
    bottom: -20%;
    opacity: 0;
  }

  .card:hover {
    box-shadow: rgba(0, 0, 0, 0.66) 0 30px 60px 0, inset #333 0 0 0 5px;
    outline-offset: 0;
  }

  .card:hover .card-bg {
    filter: blur(4px) brightness(80%);
    padding: 0;
    top: 0;
    left: 0;
  }

  .card:hover .slider-comp {
    opacity: 1;
    bottom: 0;
  }

  .card:hover .card-title {
    opacity: 0;
  }

</style>