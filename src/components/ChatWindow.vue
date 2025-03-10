<script lang="ts" setup>
interface ChatMsg {
  message: string;
  sender: "user" | "bot";
}

const props = defineProps<{
  messages: ChatMsg[];
}>();

const messagesEnd = ref<HTMLElement | null>(null);

function scrollToBottom() {
  messagesEnd.value?.scrollIntoView({ behavior: "smooth" });
}

onMounted(scrollToBottom);

watch(
  () => props.messages,
  () => scrollToBottom(),
  { deep: true }
);
</script>

<template>
  <div class="flex-1 flex flex-col overflow-hidden">
    <div class="flex-1 overflow-y-auto p-4">
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
