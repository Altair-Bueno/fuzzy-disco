<template>
  <div class="card-wrap">
    <div class="card">
      <div :style="cssVars" class="card-bg"></div>
      <h1 class="card-title">{{ title }}</h1>
      <div class="card-play">
          <i class="far fa-circle"></i>
          <i class="fas fa-play"></i>
      </div>
      <div class="card-elems">
        <button class="card-slide">
          <button class="card-slide-reverse">
            <i class="fas fa-angle-double-up"></i>
          </button>
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "CardItem",
  data() {
    return {
      title: "Killua and Gon",
      color: "red",
      image: require("../assets/sample-image.png"),
    }
  },
  computed: {
    cssVars() {
      return  {
        '--card-background': "url(" + this.image + ")",
      }
    }
  }
}
</script>

<style scoped>
  .card-wrap {
    margin: 10px;
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

  .card-play {
    position: absolute;
    top: -15%;
    left: 50%;
    display: flex;
    justify-content: center;
    align-items: center;
    color: #444444;
    cursor: pointer;
    transition: 350ms;
  }

  .fa-circle {
    position: absolute;
    font-size: 80px;
    border-radius: 50%;
    box-shadow: rgba(0, 0, 0, 0.66) 0 30px 60px 0, inset #333 0 0 0 5px;
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

  .card-elems {
    position: absolute;
    opacity: 1;
    bottom: 0;
    padding: 20px;
  }

  .card-slide {
    position: absolute;
    opacity: 0;
    background-color: whitesmoke;
    bottom: 5px;
    left: 5px;
    height: 0;
    width: 230px;
    align-items: center;
    justify-content: center;
    border: none;
    transition: 400ms;
    animation: slide-up-reverse 250ms ease-out;
  }

  .card-slide-reverse {
    border: none;
    color: #2c3e50;
    background-color: rgba(0,0,0,0);
    cursor: pointer;
  }

  .fa-angle-double-up {
    font-size: x-large;
    opacity: 0;
    transition: 400ms;
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

  .card:hover .card-slide {
    opacity: 1;
    border-radius: 0 0 30px 30px;
    height: 70px;
    cursor: pointer;
  }

  .card-slide:focus {
    opacity: 1;
    animation: slide-up 250ms ease-out;
    animation-fill-mode: forwards;
  }
  .card-slide:focus .fa-angle-double-up {
    opacity: 1;
  }
  @keyframes slide-up {
    0% {
      border-radius: 0 0 30px 30px;
      height: 70px;
    }
    100% {
      height: 310px;
      border-radius: 30px;
    }
  }
  @keyframes slide-up-reverse {
    0% {
      height: 310px;
      border-radius: 30px;
    }
    100% {
      border-radius: 0 0 30px 30px;
      height: 70px;
    }
  }

  .card:hover .fa-angle-double-up {
    opacity: 1;
  }

  .card:hover .card-title {
    opacity: 0;
  }

</style>