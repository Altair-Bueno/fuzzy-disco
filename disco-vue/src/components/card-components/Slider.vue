<template>
  <div class="card-elems">
    <button @click="slideUp($event)" class="card-slide">
      <button class="card-slide-reverse">
        <i class="fas fa-angle-double-up"></i>
      </button>
      <CardInfo class="reverse"></CardInfo>
    </button>
  </div>
</template>

<script>
import CardInfo from "@/components/card-components/CardInfo";
export default {
  name: "Slider",
  components: {CardInfo},
  methods: {
    slideUp(event) {
      const slide = document.getElementsByClassName("card-slide")[0];
      const arrow = document.getElementsByClassName("card-slide-reverse")[0];
      const reverse = document.getElementsByClassName("reverse")[0];
      if(event.target.classList.item(0) === "card-slide") {
        slide.parentElement.style.height = "97%";
        slide.style.borderRadius = "30px";
        slide.style.cursor = "default";
        slide.style.justifyContent = "flex-start";
        slide.style.flexDirection = "column";
        arrow.style.top = "50px"
        arrow.style.transform = "rotate(180deg)";
        reverse.style.display = "flex";
      } else {
        slide.parentElement.style.height = "20%";
        slide.style.borderTopLeftRadius = "0";
        slide.style.borderTopRightRadius = "0";
        slide.style.alignItems = "center";
        slide.style.justifyContent = "center";
        slide.style.cursor = "pointer";
        arrow.style.transform = "rotate(0deg)";
        reverse.style.display = "none";
      }
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

  .card-slide-reverse {
    color: #2c3e50;
    background-color: rgba(0,0,0,0);
    font-size: x-large;
    cursor: pointer;
    border: none;
    transition: 400ms;
  }

  .reverse {
    display: none;
  }

</style>