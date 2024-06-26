<script setup>
import { ref } from 'vue';
import { day2_backend } from 'declarations/day2_backend/index';
import Blog from './components/Blog.vue';
let greeting = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  const numer = target.querySelector('#numer').value;
  await day2_backend.greet(name, Number(numer)).then((response) => {
    greeting.value = response;
  });
}
</script>

<template>
  <main class ="container mx=auto">
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <section id="greeting">{{ greeting }}</section>
    <Blog />
  </main>
</template>
