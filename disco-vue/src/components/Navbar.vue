<template>
  <div>
    <nav class="navbar">
      <ul class="navbar-nav">
        <li class="nav-item">
          <a class="nav-link" href="/">
            <img class="nav-logo" src="../assets/logo.png">
          </a>
        </li>
        <li class="nav-item">
          <a class="nav-link">
            Home
          </a>
        </li>
        <li class="nav-item">
          <a class="nav-link">
            Explore
          </a>
        </li>
        <li class="nav-item">
          <div class="search-cont">
            <input class="search-box" placeholder="Search">
          </div>
        </li>
        <li class="nav-item">
          <a class="nav-link">
            User
          </a>
        </li>
      </ul>
    </nav>
  </div>
</template>

<script>
export default {
  name: "Navbar"
}
</script>

<style scoped>
  .navbar {
    position: relative;
    display: flex;
    top: 0;
    height: 4rem;
    width: 100vw;
    background-color: rgb(40, 42, 53);
    color: whitesmoke;
    font-family: "Open Sans", sans-serif;
  }

  .navbar-nav {
    display: flex;
    align-items: center;
    margin: 0;
    padding: 0;
    width: 100%;
    list-style: none;
  }

  .nav-link {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 4rem;
    width: 6rem;
    text-decoration: none;
    transition: 250ms;
  }

  .nav-link:hover {
    background-color: rgb(30, 32, 53);
    cursor: pointer;
    opacity: 0.7;
    color: rgba(0, 250, 154, 1);
    box-shadow: rgba(0, 250, 154, 0.66) 0 0 10px 0;
  }

  .nav-link:hover .nav-logo {

  }

  .search-cont {
    position: relative;
  }

  .search-box {
    margin: 0 1.5rem;
    border: none;
    border-bottom: 1px solid #ccc;
    height: 1.5rem;
    width: 5rem;
    background-color: rgb(40, 42, 53);
    color: whitesmoke;
    transition: 300ms;
    font-family: "Open Sans", sans-serif;
    font-size: 16px;
    opacity: 0.9;
    font-weight: lighter;
  }

  .search-box:focus {
    outline: none;
    width: 15rem;
    border-color: rgba(0, 250, 154, 0.66);
  }

  .search-box::placeholder {
    opacity: 0.3;
    font-weight: lighter;
    font-size: 14px;
    font-family: "Open Sans", sans-serif;
  }

  .nav-logo {
    font-size: x-large;
    width: 2rem;
    margin: 0 1.5rem;
  }

  .nav-item:last-child {
    margin-left: auto;
  }

</style>