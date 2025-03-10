<script setup lang="ts">
interface ChatMsg {
  message: string;
  sender: "user" | "bot";
}

const messages = ref<ChatMsg[]>([]);

const isLoading = ref(false);

async function handleSendMessage(msg: string) {
  messages.value.push({ message: msg, sender: "user" });

  isLoading.value = true;

  try {
    setTimeout(() => {
      messages.value.push({
        message:
          "This is a placeholder response. In the full implementation, this would come from Ollama.",
        sender: "bot",
      });
      isLoading.value = false;
    }, 1000);
  } catch (error) {
    messages.value.push({
      message: "Sorry, there was an error processing your request.",
      sender: "bot",
    });
    isLoading.value = false;
  }
}
</script>

<template>
  <main class="min-h-screen flex flex-col text-white">
    <Header />
    <div class="pt-16 pb-20">
      <ChatWindow :messages="messages" />
    </div>
    <ChatInput @send="handleSendMessage" :disabled="isLoading" />
  </main>
</template>
