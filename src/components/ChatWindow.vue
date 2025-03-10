<script lang="ts" setup>
interface ChatMsg {
  message: string;
  sender: "user" | "assistant";
  thinking?: string;
}

const props = defineProps<{
  messages: ChatMsg[];
}>();

const chatContainer = ref<HTMLElement | null>(null);
const messagesEnd = ref<HTMLElement | null>(null);
const lastScrollHeight = ref(0);
const shouldAutoScroll = ref(true); // Default to true

function handleScroll() {
  if (!chatContainer.value) return;

  const { scrollTop, scrollHeight, clientHeight } = chatContainer.value;
  const atBottom = Math.abs(scrollHeight - scrollTop - clientHeight) < 50;

  // Update auto-scroll flag based on user behavior
  if (!atBottom && scrollHeight > lastScrollHeight.value) {
    // User has scrolled up while content is growing - disable auto-scroll
    shouldAutoScroll.value = false;
  } else if (atBottom) {
    // User is at bottom - re-enable auto-scroll
    shouldAutoScroll.value = true;
  }

  // Save current scroll height
  lastScrollHeight.value = scrollHeight;
}

async function scrollToBottom() {
  await nextTick();
  messagesEnd.value?.scrollIntoView({ behavior: "smooth" });
}

// Manually scroll to bottom (for button)
function forceScrollToBottom() {
  shouldAutoScroll.value = true;
  scrollToBottom();
}

onMounted(async () => {
  chatContainer.value?.addEventListener("scroll", handleScroll);
  await scrollToBottom();
});

// Only watch for NEW messages (not content changes)
watch(
  () => props.messages.length,
  async () => {
    // Only scroll on new messages if auto-scroll is enabled
    if (shouldAutoScroll.value) {
      await scrollToBottom();
    }
  }
);
</script>

<template>
  <div class="flex-1 flex flex-col overflow-hidden relative">
    <div
      ref="chatContainer"
      class="flex-1 overflow-y-auto p-4"
      @scroll="handleScroll"
    >
      <div class="max-w-3xl mx-auto w-full">
        <div class="flex flex-col space-y-4">
          <ChatMessage
            v-for="(msg, index) in messages"
            :key="index"
            :message="msg.message"
            :sender="msg.sender"
            :thinking="msg.thinking"
          />
        </div>
        <div
          v-if="messages.length === 0"
          class="text-center text-zinc-400 text-2xl mt-8"
        >
          No messages yet.
        </div>
        <div ref="messagesEnd" class="h-4"></div>
      </div>
    </div>

    <!-- Scroll to bottom button (appears when not auto-scrolling) -->
    <button
      v-if="!shouldAutoScroll && messages.length > 0"
      @click="forceScrollToBottom"
      class="absolute bottom-4 right-4 bg-zinc-800 hover:bg-zinc-700 rounded-full p-2 shadow-lg border border-zinc-700"
      title="Scroll to bottom"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-5 w-5"
        viewBox="0 0 20 20"
        fill="currentColor"
      >
        <path
          fill-rule="evenodd"
          d="M14.707 12.707a1 1 0 01-1.414 0L10 9.414l-3.293 3.293a1 1 0 01-1.414-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 010 1.414z"
          clip-rule="evenodd"
        />
      </svg>
    </button>
  </div>
</template>
