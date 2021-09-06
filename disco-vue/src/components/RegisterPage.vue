<template>
  <div class="object-cont">
    <div class="form-cont">
      <form class="login-form">
        <h1>Sign Up</h1>
        <label for="email">Email</label><br>
        <input class="input-text" type="email" id="email"><br><br>
        <label for="alias">Username</label><br>
        <input class="input-text" type="email" id="alias"><br><br>
        <label for="pwd">Password</label><br>
        <input class="input-text" type="email" id="pwd"><br><br>
        <label for="repeat-pwd">Repeat Password</label><br>
        <input class="input-text" type="password" id="repeat-pwd"><br><br>
        <input class="submit-btn" type="submit" value="Register">
      </form>
      <div class="login-text">
        <p>Already have an account? <a class="login-link" href="#">Login here</a></p>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "RegisterPage"
}
</script>

<style scoped>
  * {
    --login-border: rgba(0, 250, 154, 0.66)
  }

  .object-cont {
    font-family: "Open Sans", sans-serif;
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 5rem;
  }

  .form-cont {
    position: relative;
    border-radius: 25px;
    color: whitesmoke;
    border: 1px solid var(--login-border);
    padding: 5rem;
    height: 30rem;
    width: 20rem;
    box-shadow: 5px 5px 25px 10px rgba(0, 0, 0, 0.5);
  }

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

  .submit-btn {
    font-family: "Open Sans", sans-serif;
    color: #444444;
    font-weight: bold;
    font-size: 1rem;
    border: none;
    padding-left: 2.5rem;
    padding-right: 2.5rem;
    height: 2rem;
    width: 9rem;
    cursor: pointer;
    background-color: whitesmoke;
    border-radius: 25px;
    transition: 300ms;
  }

  .submit-btn:hover {
    background-color: var(--login-border);
    width: 15rem;
  }

  h1:hover {
    cursor: default;
  }

  .login-text {
    margin-top: 6rem;
  }

  .login-link {
    color: var(--login-border);
  }
</style>