<script setup lang="ts">
interface ChatMsg {
  message: string;
  sender: "user" | "assistant" | "system";
  thinking?: string;
}

const messages = ref<ChatMsg[]>([]);
const isLoading = ref(false);
const isStream = ref(true);
const currentModel = ref("");

let shouldBreak = false;

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
        messages: [...messageHistory],
        stream: isStream.value,
      }),
    });

    if (!response.ok) {
      throw new Error(`Error: ${response.status} ${response.statusText}`);
    }

    if (!isStream.value) {
      // Cancel stream
      const data = await response.json();

      // Parse the message content for thinking tags in non-streaming mode
      let messageContent = data.message.content;
      let thinkingContent = "";

      // Extract thinking content from between <think> tags
      const thinkMatch = messageContent.match(/<think>([\s\S]*?)<\/think>/);
      if (thinkMatch) {
        thinkingContent = thinkMatch[1];
        // Remove the thinking tags from the message content
        messageContent = messageContent.replace(
          /<think>[\s\S]*?<\/think>/g,
          ""
        );
      }

      messages.value[botMessageIndex] = {
        message: messageContent,
        sender: "assistant",
        thinking: thinkingContent,
      };
      return;
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
      if (shouldBreak) {
        shouldBreak = false;
        break;
      }

      const chunk = new TextDecoder().decode(value);

      try {
        const lines = chunk.split("\n");

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
        // Constantly spams "Unexpected end of JSON input" due to the stream not being complete yet
        // console.error("Error parsing JSON:", e);
      }
    }
  } catch (error) {
    console.error("Error calling Ollama:", error);

    messages.value[botMessageIndex] = {
      message: `Sorry, I encountered an error: ${
        error instanceof Error ? error.message : "Unknown error"
      }.\nMake sure Ollama is running locally.`,
      sender: "system",
    };
  } finally {
    isLoading.value = false;
  }
}

function clearMessages() {
  messages.value = [];
  shouldBreak = true;
}

function handleCancel() {
  isLoading.value = false;
  shouldBreak = true;
}
</script>

<template>
  <main class="min-h-screen flex flex-col text-white bg-zinc-900">
    <Header
      @model-change="(model) => (currentModel = model)"
      @stream-toggle="(stream: boolean) => (isStream = stream)"
      @clear-messages="clearMessages"
      :current-model="currentModel"
    />

    <div class="flex-1 pt-16 pb-20">
      <ChatWindow :messages="messages" />
    </div>

    <ChatInput
      @send="handleSendMessage"
      @cancel="handleCancel"
      :disabled="isLoading"
    />
  </main>
</template>
