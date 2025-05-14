<script lang="ts" setup>
interface MessageProperties {
  message: string;
  sender: "user" | "assistant" | "system";
  thinking?: string;
}

const props = defineProps<MessageProperties>();
const showThinking = ref(false);

const senderClass = computed(() =>
  props.sender === "user"
    ? "self-end bg-zinc-800 text-white"
    : "self-start text-white"
);

const hasThinking = computed(
  () => !!props.thinking && props.thinking.trim().length > 0
);

function toggleThinking() {
  showThinking.value = !showThinking.value;
}
</script>

<template>
  <div
    :class="[
      'max-w-lg p-3 my-2 rounded-xl text-xl',
      senderClass,
      { 'bg-red-800/20 border-1 border-red-500': sender === 'system' },
    ]"
  >
    <!-- System message indicator -->
    <div v-if="sender === 'system'" class="flex items-center mb-2 text-red-300">
      <span class="mr-2">⚠️</span>
      <span class="font-semibold">System Message</span>
    </div>

    <!-- Thinking indicator (only for assistant messages with thinking content) -->
    <div v-if="hasThinking && sender === 'assistant'" class="mb-2">
      <button
        @click="toggleThinking"
        class="text-xs bg-zinc-600/50 hover:bg-zinc-600 py-0.5 px-2 rounded text-zinc-300 flex items-center"
      >
        <span
          class="inline-block animate-pulse mr-1 text-blue-500 text-xxl"
          v-if="!showThinking"
          >●</span
        >
        <span v-if="!showThinking">Thinking</span>
        <span v-else>Hide thinking</span>
      </button>

      <!-- Thinking content (collapsible) -->
      <div v-if="showThinking" class="text-sm text-zinc-300 border-l-2 pl-2">
        <pre class="whitespace-pre-wrap font-mono">{{ thinking }}</pre>
      </div>
    </div>

    <!-- Regular message content -->
    <div>
      <pre
        class="whitespace-pre-wrap font-normal"
        style="font-family: inherit"
        >{{ message }}</pre
      >
    </div>
  </div>
</template>
