<script setup lang="ts">
interface ChatMsg {
  message: string;
  sender: "user" | "assistant";
  thinking?: string;
}

const messages = ref<ChatMsg[]>([]);
const isLoading = ref(false);
const currentModel = ref("");

async function handleSendMessage(msg: string) {
  messages.value.push({ message: msg, sender: "user" });

  const botMessageIndex = messages.value.length;
  messages.value.push({
    message: "",
    sender: "assistant",
    thinking: "",
  });

  isLoading.value = true;

  try {
    const messageHistory = messages.value.slice(0, -1).map((m) => ({
      role: m.sender === "user" ? "user" : "assistant",
      content: m.message,
    }));

    const apiUrl =
      import.meta.env.VITE_OLLAMA_API_URL || "http://localhost:11434";
    const response = await fetch(`${apiUrl}/api/chat`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        model: currentModel.value,
        messages: [...messageHistory, { role: "user", content: msg }],
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

    let streamedResponse = "";
    let thinkingContent = "";
    let inThinkingBlock = false;

    while (true) {
      const { done, value } = await reader.read();

      if (done) break;

      const chunk = new TextDecoder().decode(value);

      try {
        const lines = chunk.split("\n").filter((line) => line.trim() !== "");

        for (const line of lines) {
          const data = JSON.parse(line);
          if (data.message?.content) {
            const content = data.message.content;

            // Check for <think> tags in the content
            if (content.includes("<think>")) {
              inThinkingBlock = true;
              const thinkParts = content.split("<think>");
              streamedResponse += thinkParts[0];
              thinkingContent += thinkParts[1];
            } else if (content.includes("</think>")) {
              inThinkingBlock = false;
              const endThinkParts = content.split("</think>");
              thinkingContent += endThinkParts[0];
              streamedResponse += endThinkParts[1] || "";
            } else if (inThinkingBlock) {
              thinkingContent += content;
            } else {
              streamedResponse += content;
            }

            // Update the message in real-time
            messages.value[botMessageIndex] = {
              message: streamedResponse,
              thinking: thinkingContent,
              sender: "assistant",
            };
          }
        }
      } catch (e) {
        console.error("Error parsing stream chunk:", e);
      }
    }
  } catch (error) {
    console.error("Error calling Ollama:", error);

    messages.value[botMessageIndex] = {
      message: `Sorry, I encountered an error: ${
        error instanceof Error ? error.message : "Unknown error"
      }. Make sure Ollama is running locally.`,
      sender: "assistant",
    };
  } finally {
    isLoading.value = false;
  }
}

function clearMessages() {
  messages.value = [];
}
</script>

<template>
  <main class="min-h-screen flex flex-col text-white bg-zinc-900">
    <Header
      @model-change="(model) => (currentModel = model)"
      @clear-messages="clearMessages"
      :current-model="currentModel"
    />
    <div class="flex-1 pt-16 pb-20">
      <ChatWindow :messages="messages" />
    </div>
    <ChatInput @send="handleSendMessage" :disabled="isLoading" />
  </main>
</template>
