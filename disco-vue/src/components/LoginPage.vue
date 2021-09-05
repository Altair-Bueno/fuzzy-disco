<template>
  <div class="object-cont">
    <div class="form-cont">
      <form class="login-form">
        <h1>Login</h1>
        <label for="username">Username or Email</label><br>
        <input class="input-text" type="email" id="username"><br><br>
        <label for="pwd">Password</label><br>
        <input class="input-text" type="password" id="pwd"><br><br>
        <input class="submit-btn" type="submit" value="Login">
      </form>
      <br>
      <div class="register-text">
        <p>Dont have an acoount yet? <a class="register-link" href="#">Register now</a></p>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "LoginPage"
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
    height: 25rem;
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
    width: 5rem;
    height: 2rem;
    cursor: pointer;
    background-color: whitesmoke;
    border-radius: 25px;
    transition: 300ms;
  }

  .submit-btn:hover {
    background-color: var(--login-border);
    width: 10rem;
  }

  h1:hover {
    cursor: default;
  }

  .register-text {
    margin-top: 6rem;
  }

  .register-link {
    color: var(--login-border);
  }

</style>