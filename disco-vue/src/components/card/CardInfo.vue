<template>
  <div class="text">
    <p class="caption">
      {{ caption }}
    </p>
    <a href="https://example.org/" class="author">
      <h3>
        {{ author }}
      </h3>
    </a>

  </div>
</template>

<script>
export default {
  name: "CardInfo",
  props: {
    author: {
      type: String,
      required: true
    },
    caption: {
      type: String,
      required: true
    }
  },
}
</script>

<style scoped>
  .text {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    height: 100%;
    --author-color: #2c3e50;
  }

  .caption {
    font-size: 18px;
    font-family: "Open Sans", sans-serif;
    font-weight: lighter;
  }
  .author {
    font-size: 24px;
    font-family: "Open Sans", sans-serif;
    font-weight: bolder;
  }

  .author:link {
    text-decoration: none;
  }

  .author:visited {
    text-decoration: none;
    color: var(--author-color);
  }
</style>