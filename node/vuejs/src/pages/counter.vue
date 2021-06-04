<template>
  <div>
    <img alt="Vue logo" src="../assets/logo.png">
    <p>Counter page</p>
    <p>The button above has been clicked {{ counter }} times.</p>
    <ul>
      <li><button @click="incriment">Add 1</button></li>
      <li><button @click="decriment">Take 1</button></li>
    </ul>
    <a href="/">home</a>
  </div>
</template>

<script>

export default {
  name: 'Counter',
  data() {
    return {
      counter: 0
    }
  },
  methods: {
    incriment() {
      this.counter++;
    },
    decriment() {
      this.counter--;
    }
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
