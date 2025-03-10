<script setup lang="ts">
interface ChatMsg {
  message: string;
  sender: "user" | "bot";
}

const messages = ref<ChatMsg[]>([]);
const isLoading = ref(false);
const currentModel = ref("huihui_ai/qwq-abliterated");

async function handleSendMessage(msg: string) {
  // Add user message to chat
  messages.value.push({ message: msg, sender: "user" });

  // Create an empty message for streaming the response
  const botMessageIndex = messages.value.length;
  messages.value.push({ message: "", sender: "bot" });

  isLoading.value = true;

  try {
    // Call Ollama API with streaming enabled
    const response = await fetch("http://localhost:11434/api/generate", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        model: currentModel.value,
        prompt: msg,
        stream: true,
      }),
    });

    if (!response.ok) {
      throw new Error(`Error: ${response.status} ${response.statusText}`);
    }

    const reader = response.body?.getReader();
    if (!reader) {
      throw new Error("ReadableStream not supported");
    }

    // Process the stream
    let streamedResponse = "";

    while (true) {
      const { done, value } = await reader.read();

      if (done) {
        break;
      }

      // Decode the stream chunk
      const chunk = new TextDecoder().decode(value);

      try {
        // Each line is a separate JSON object
        const lines = chunk.split("\n").filter((line) => line.trim() !== "");

        for (const line of lines) {
          const data = JSON.parse(line);
          streamedResponse += data.response;

          // Update the current bot message with accumulated text
          messages.value[botMessageIndex].message = streamedResponse;
        }
      } catch (e) {
        console.error("Error parsing stream chunk:", e);
      }
    }
  } catch (error) {
    console.error("Error calling Ollama:", error);

    // Replace the empty bot message with an error message
    messages.value[botMessageIndex] = {
      message: `Sorry, I encountered an error: ${
        error instanceof Error ? error.message : "Unknown error"
      }. Make sure Ollama is running locally.`,
      sender: "bot",
    };
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <main class="min-h-screen flex flex-col text-white bg-zinc-900">
    <Header />
    <div class="flex-1 pt-16 pb-20">
      <ChatWindow :messages="messages" />
    </div>
    <ChatInput @send="handleSendMessage" :disabled="isLoading" />
  </main>
</template>
