<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="min-h-screen bg-gray-900 flex flex-col items-center justify-center p-4">
    <h1 class="text-4xl font-bold text-purple-400 mb-8">Скуфидрон</h1>

    <form class="flex flex-col items-center gap-4 w-full max-w-md" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..."
        class="w-full px-4 py-2 rounded-lg bg-gray-800 text-white border border-gray-700 focus:border-purple-500 focus:ring-2 focus:ring-purple-500 focus:outline-none transition-all" />
      <button type="submit"
        class="px-6 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transform hover:scale-105 transition-all duration-200 font-medium">
        Greet
      </button>
    </form>
    <p class="mt-6 text-gray-300 text-lg">{{ greetMsg }}</p>
  </main>
</template>

<style></style>
