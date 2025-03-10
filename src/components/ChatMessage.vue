<script lang="ts" setup>
interface Props {
  message: string;
  sender: "user" | "assistant";
  thinking?: string;
}

const props = defineProps<Props>();
const showThinking = ref(false);

const senderClass = computed(() =>
  props.sender === "user"
    ? "self-end bg-zinc-800 text-white"
    : "self-start text-white"
);

const hasThinking = computed(
  () => !!props.thinking && props.thinking.length > 0
);

function toggleThinking() {
  showThinking.value = !showThinking.value;
}
</script>

<template>
  <div :class="['max-w-lg p-3 my-2 rounded-xl text-lg', senderClass]">
    <!-- Regular message content -->
    {{ message }}

    <!-- Thinking toggle button (only for bot messages with thinking content) -->
    <div v-if="hasThinking" class="mt-2">
      <button
        @click="toggleThinking"
        class="text-xs bg-zinc-700 hover:bg-zinc-600 py-1 px-2 rounded-md text-zinc-300 flex items-center"
      >
        <span v-if="!showThinking">
          <span class="inline-block animate-pulse mr-1">●</span>
          View thinking
        </span>
        <span v-else>Hide thinking</span>
      </button>

      <!-- Thinking content (collapsible) -->
      <div
        v-if="showThinking"
        class="mt-2 p-3 bg-zinc-700/50 border border-zinc-600 rounded-md overflow-auto text-sm"
      >
        <pre class="whitespace-pre-wrap text-zinc-300">{{ thinking }}</pre>
      </div>
    </div>
  </div>
</template>
