<script lang="ts" setup>
import { ref } from "vue";

const message = ref("");
const emit = defineEmits<{ (e: "send", msg: string): void }>();

function handleSubmit(e: Event) {
  e.preventDefault();
  if (message.value.trim()) {
    emit("send", message.value.trim());
    message.value = "";
  }
}
</script>

<template>
  <div class="w-full">
    <form
      @submit="handleSubmit"
      class="flex p-4 max-w-1/2 mx-auto bottom-1/6 fixed left-0 right-0"
    >
      <input
        v-model="message"
        type="text"
        placeholder="Type your message..."
        class="flex-1 px-3 py-2 rounded-xl shadow-md border border-zinc-600 bg-zinc-700 text-white placeholder-zinc-400 focus:outline-none focus:border-blue-400"
      />
      <button
        type="submit"
        class="ml-2 px-4 py-2 bg-blue-600 text-white rounded-xl hover:bg-blue-700 transition-colors shadow-md"
      >
        Send
      </button>
    </form>
  </div>
</template>
