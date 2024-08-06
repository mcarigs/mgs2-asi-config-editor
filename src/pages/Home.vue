<script setup lang="ts">
// This component currently doesn't have any TypeScript logic
</script>

<template>
  <div class="home">
    <h1>Home</h1>
    <div class="content">
      <p>MGS2 ASI Mod Config Editor</p>
      <span class="author">Carigs</span>
    </div>
  </div>
</template>

<style scoped>
.home {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  text-align: center;
}

h1 {
  font-size: 2.5em;
  margin-bottom: 20px;
}

.content {
  font-size: 1.2em;
}

.author {
  display: block;
  margin-top: 10px;
  font-style: italic;
}
</style>
