<script lang="ts" setup>
const message = ref("");
const emit = defineEmits<{ (e: "send", msg: string): void }>();
const props = defineProps<{ disabled?: boolean }>();

function handleSubmit(e: Event) {
  e.preventDefault();
  if (message.value.trim() && !props.disabled) {
    emit("send", message.value.trim());
    message.value = "";
  }
}
</script>

<template>
  <div class="w-full fixed bottom-0 left-0 right-0 bg-zinc-900 px-4 py-3">
    <form @submit="handleSubmit" class="max-w-3xl mx-auto flex">
      <input
        v-model="message"
        type="text"
        placeholder="Type your message..."
        :disabled="disabled"
        class="flex-1 px-3 py-2 rounded-xl shadow-md border border-zinc-600 bg-zinc-700 text-white placeholder-zinc-400 focus:outline-none focus:border-blue-400"
      />
      <button
        type="submit"
        :disabled="disabled"
        class="ml-2 px-4 py-2 bg-blue-600 text-white rounded-xl hover:bg-blue-700 transition-colors shadow-md disabled:bg-blue-800 disabled:opacity-50"
      >
        Send
      </button>
    </form>
  </div>
</template>
