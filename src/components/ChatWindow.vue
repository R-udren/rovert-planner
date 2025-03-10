<script lang="ts" setup>
import { nextTick, onMounted, ref, watch } from "vue";
import ChatMessage from "./ChatMessage.vue";

interface ChatMsg {
  message: string;
  sender: "user" | "bot";
}

const props = defineProps<{
  messages: ChatMsg[];
}>();

const chatContainer = ref<HTMLElement | null>(null);
const messagesEnd = ref<HTMLElement | null>(null);

async function scrollToBottom() {
  await nextTick();

  messagesEnd.value?.scrollIntoView({ behavior: "smooth" });

  if (chatContainer.value) {
    chatContainer.value.scrollTop = chatContainer.value.scrollHeight;
  }
}

onMounted(async () => {
  await scrollToBottom();
});

watch(
  () => props.messages,
  async () => {
    await scrollToBottom();
  },
  { deep: true }
);
</script>

<template>
  <div class="flex-1 flex flex-col overflow-hidden">
    <div ref="chatContainer" class="flex-1 overflow-y-auto p-4">
      <div class="max-w-3xl mx-auto w-full">
        <div class="flex flex-col space-y-4">
          <ChatMessage
            v-for="(msg, index) in messages"
            :key="index"
            :message="msg.message"
            :sender="msg.sender"
          />
        </div>
        <div ref="messagesEnd" class="h-4"></div>
      </div>
    </div>
  </div>
</template>
