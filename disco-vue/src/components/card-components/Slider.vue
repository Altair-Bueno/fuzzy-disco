<template>
  <div class="card-elems" :class="{'card-elems-open': isOpen}">
    <button @click="isOpen = !isOpen" class="card-slide" :class="{'card-slide-open': isOpen}">
      <button class="card-slide-reverse" :class="{'card-slide-reverse-open': isOpen}">
        <i class="fas fa-angle-double-up"></i>
      </button>
      <CardInfo :author="card.author_id" :caption="card.caption" class="reverse" :class="{'reverse-open': isOpen}"></CardInfo>
    </button>
  </div>
</template>

<script>
import CardInfo from "@/components/card-components/CardInfo";
export default {
  name: "Slider",
  components: {CardInfo},
  props: {
    card: {}
  },
  data() {
    return {
      isOpen: false
    }
  }
}
</script>

<style scoped>
  .card-elems {
    position: absolute;
    opacity: 1;
    bottom: 0;
    margin: 5px;
    height: 20%;
    width: 96%;
    transition: 400ms;
  }

  .card-elems-open {
    position: absolute;
    opacity: 1;
    bottom: 0;
    margin: 5px;
    width: 96%;
    transition: 400ms;
    height: 97%;
  }

  .card-slide {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 1;
    background-color: whitesmoke;
    cursor: pointer;
    width: 100%;
    height: 100%;
    border: none;
    border-radius: 0 0 30px 30px;
    transition: 400ms;
  }

  .card-slide-open {
    position: relative;
    display: flex;
    align-items: center;
    opacity: 1;
    background-color: whitesmoke;
    width: 100%;
    height: 100%;
    border: none;
    transition: 400ms;
    border-radius: 30px;
    cursor: default;
    justify-content: flex-start;
    flex-direction: column;
  }

  .card-slide-reverse {
    color: #2c3e50;
    background-color: rgba(0,0,0,0);
    font-size: x-large;
    cursor: pointer;
    border: none;
    transition: 400ms;
  }

  .card-slide-reverse-open {
    color: #2c3e50;
    background-color: rgba(0,0,0,0);
    font-size: x-large;
    cursor: pointer;
    border: none;
    transition: 400ms;
    top: 50px;
    transform: rotate(180deg);
  }

  .reverse {
    display: none;
  }

  .reverse-open {
    display: flex;
  }

</style>