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
          <RouterLink to="/home" class="nav-link">
            Home
          </RouterLink>
        </li>
        <li class="nav-item">
          <RouterLink to="/new-post" class="nav-link">
            New Post
          </RouterLink>
        </li>
        <li class="nav-item">
          <div class="search-cont">
            <input class="search-box" placeholder="Search">
          </div>
        </li>
        <li class="nav-item">
          <button class="drop-btn">User</button>
          <div class="drop-content">
            <RouterLink to="/user/sample-user/profile">Profile</RouterLink>
            <RouterLink to="/user/sample-user/settings">Settings</RouterLink>
          </div>
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
  * {
    --navbar-color: rgb(40, 42, 53);
    --navbar-hover: rgb(30, 32, 53)
  }
  .navbar {
    position: relative;
    display: flex;
    height: 4rem;
    width: 100vw;
    background-color: var(--navbar-color);
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
    color: whitesmoke;
    align-items: center;
    justify-content: center;
    height: 4rem;
    width: 6rem;
    text-decoration: none;
    transition: 250ms;
  }

  .nav-link:hover {
    background-color: var(--navbar-hover);
    cursor: pointer;
    opacity: 0.7;
    color: rgba(0, 250, 154, 1);
    box-shadow: rgba(0, 250, 154, 0.66) 0 0 10px 0;
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
    background-color: var(--navbar-color);
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

  .drop-btn {
    background-color: var(--navbar-color);
    color: white;
    padding: 16px;
    font-size: 16px;
    border: none;
    cursor: pointer;
    height: 4rem;
    width: 10rem;
    transition: 250ms;
  }

  .drop-content {
    display: none;
    position: absolute;
    background-color: rgb(255,250,240);
    min-width: 10rem;
    box-shadow: 0 8px 16px 0 rgba(0,0,0,0.2);
    z-index: 1;
    right: 0;
    transition: 250ms;
  }

  .drop-content a {
    color: black;
    padding: 12px 16px;
    text-decoration: none;
    display: block;
  }

  .drop-content a:hover {
    background-color: rgb(245,240,230)
  }

  .nav-item:hover .drop-content {
    display: block;
  }

  .nav-item:hover .drop-btn {
    background-color: var(--navbar-hover);
    cursor: pointer;
    opacity: 0.7;
    color: rgba(0, 250, 154, 1);
    box-shadow: rgba(0, 250, 154, 0.66) 0 0 10px 0;
  }

</style>