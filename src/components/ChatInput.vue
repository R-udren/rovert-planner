<script lang="ts" setup>
const message = ref("");
const hiddenTextRef = ref<HTMLDivElement | null>(null);
const inputAreaRef = ref<HTMLDivElement | null>(null);
const emit = defineEmits<{
  (e: "send", msg: string): void;
  (e: "cancel"): void;
}>();
const props = defineProps<{ disabled?: boolean }>();

function handleSubmit(e: Event) {
  e.preventDefault();
  if (message.value.trim() && !props.disabled) {
    emit("send", message.value.trim());
    message.value = "";
    // Reset the input area
    if (inputAreaRef.value) {
      inputAreaRef.value.innerText = "";
    }
    inputAreaRef.value?.blur();

    // Reset height
    if (inputAreaRef.value) {
      inputAreaRef.value.style.height = "auto";
    }
  }
}

function handleKeydown(e: KeyboardEvent) {
  // Send on Ctrl+Enter or Cmd+Enter
  if (!e.shiftKey && e.key === "Enter") {
    e.preventDefault();
    handleSubmit(e);
    return;
  }

  // Handle enter key without modifier for creating a newline
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    const selection = window.getSelection();
    const range = selection?.getRangeAt(0);
    if (range) {
      const newline = document.createTextNode("\n");
      range.insertNode(newline);
      range.setStartAfter(newline);
      range.setEndAfter(newline);
      selection?.removeAllRanges();
      selection?.addRange(range);

      if (inputAreaRef.value) {
        message.value = inputAreaRef.value.innerText;
      }

      adjustInputHeight();
    }
  }
}

function onInput(e: Event) {
  const target = e.target as HTMLDivElement;
  message.value = target.innerText;
  adjustInputHeight();
}

function adjustInputHeight() {
  nextTick(() => {
    if (hiddenTextRef.value && inputAreaRef.value) {
      const content =
        message.value === "" ? "" : message.value.replace(/\n/g, "<br>");

      hiddenTextRef.value.innerHTML = content;

      // Convert px to rem
      // Using 3.5rem (56px) as min and 18.75rem (300px) as max
      // assuming 1rem = 16px
      const height = Math.min(
        18.75, // 300px / 16 = 18.75rem
        Math.max(
          3.5, // 56px / 16 = 3.5rem
          hiddenTextRef.value.clientHeight / 16
        )
      );
      inputAreaRef.value.style.height = `${height}rem`;
    }
  });
}

watch(() => message.value, adjustInputHeight);

onMounted(() => {
  if (inputAreaRef.value) {
    inputAreaRef.value.focus();
  }
  adjustInputHeight();
});
</script>

<template>
  <div class="w-full fixed bottom-0 left-0 right-0 bg-zinc-900 px-4 py-3">
    <p v-if="props.disabled" class="text-center text-md text-zinc-400">
      Loading answer...
    </p>
    <form @submit="handleSubmit" class="max-w-3xl mx-auto flex">
      <!-- Hidden div used for measuring text height -->
      <div
        ref="hiddenTextRef"
        class="invisible absolute whitespace-pre-wrap break-words px-4 py-3 text-md w-full"
        aria-hidden="true"
      ></div>

      <!-- Actual editable content area -->
      <div
        ref="inputAreaRef"
        contenteditable="true"
        :class="[
          'flex-1 px-4 py-3 text-md rounded-xl shadow-md border border-zinc-700 bg-zinc-800',
          'text-white placeholder-zinc-400 focus:outline-none focus:ring-2 focus:ring-blue-500',
          'focus:border-transparent min-h-[3.5rem] max-h-[18.75rem] transition-all duration-200',
          'overflow-y-auto whitespace-pre-wrap break-words flex items-center',
        ]"
        @keydown="handleKeydown"
        @input="onInput"
        role="textbox"
        aria-multiline="true"
        :data-placeholder="!message ? 'Type your message...' : ''"
      ></div>

      <button
        v-if="!props.disabled"
        type="submit"
        :disabled="!message.trim()"
        class="ml-2 px-4 py-2 text-lg bg-blue-600 text-white rounded-xl hover:bg-blue-700 transition-colors shadow-md disabled:bg-blue-800 disabled:opacity-50"
      >
        Send
      </button>
      <button
        v-else
        type="button"
        @click="emit('cancel')"
        class="ml-2 px-4 py-2 text-lg bg-red-600   text-white rounded-xl hover:bg-red-700 transition-colors shadow-md"
      >
        Cancel
      </button>
    </form>
  </div>
</template>

<style scoped>
[contenteditable][data-placeholder]:empty:before {
  content: attr(data-placeholder);
  color: rgb(156 163 175);
  pointer-events: none;
}

[contenteditable][data-placeholder]:empty:focus:before {
  content: "";
}

/* minimalistic scrollbar */
::-webkit-scrollbar {
  width: 8px;
  background-color: transparent;
}
</style>
