<script lang="ts" setup>
interface Props {
  message: string;
  sender: "user" | "bot";
}

const props = defineProps<Props>();

const senderClass = computed(() =>
  props.sender === "user"
    ? "self-end bg-blue-500 text-white"
    : "self-start text-white bg-zinc-800"
);
</script>

<template>
  <div :class="['max-w-lg p-3 my-2 rounded-xl text-lg', senderClass]">
    {{ message }}
  </div>
</template>
