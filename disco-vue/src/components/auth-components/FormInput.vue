<template>
  <div>
    <label :for="identifier">{{this.field}}</label><br>
    <input v-model="input"
           @input="sendInfo"
           class="input-text"
           :class="{'filled-input': input !== '', 'invalid-input': !inputOk}"
           :type="inputType"
           :id="identifier"><br><br>
  </div>
</template>

<script>
export default {
  name: "FormInput",
  props: {
    identifier: String,
    inputType: String,
    field: String,
    inputOk: Boolean
  },
  data() {
    return {
      input: ""
    }
  },
  emit: {
    'input-update': null
  },
  methods: {
    sendInfo() {
      this.$emit('input-update', {
        update: this.input
      })
    }
  }
}
</script>

<style scoped>
  .input-text {
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

  .input-text:hover {
    outline: none;
    width: 15rem;
    border-color: var(--login-border);
  }

  .input-text:focus {
    outline: none;
    width: 15rem;
    border-color: var(--login-border);
  }

  .filled-input {
    outline: none;
    width: 15rem;
    border-color: #ccc;
  }

  .invalid-input {
    outline: none;
    width: 15rem;
    border-color: red;
  }

</style>