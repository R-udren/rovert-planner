<script setup lang="ts">
interface OllamaModel {
  name: string;
  modified_at: string;
  size: number;
  digest: string;
  details: {
    format: string;
    family: string;
    families: null | string[];
    parameter_size: string;
    quantization_level: string;
  };
}

const props = defineProps<{
  currentModel: string;
}>();

const emit = defineEmits<{
  (e: "modelChange", model: string): void;
  (e: "clearMessages"): void;
  (e: "streamToggle", streamResponse: boolean): void;
}>();

const isDropdownOpen = ref(false);
const streamMessage = ref(false);
const availableModels = ref<string[]>([]);
const error = ref<string | null>(null);
const isLoading = ref(true);

async function fetchModels() {
  try {
    isLoading.value = true;
    error.value = null;

    const apiUrl =
      import.meta.env.VITE_OLLAMA_API_URL || "http://localhost:11434";
    const response = await fetch(`${apiUrl}/api/tags`);

    if (!response.ok) {
      throw new Error(`Error: ${response.status} ${response.statusText}`);
    }

    const data = await response.json();
    availableModels.value = data.models.map((model: OllamaModel) => model.name);

    // If no model is currently selected, select the smallest one
    if (!props.currentModel && availableModels.value.length > 0) {
      const smallestModel = data.models.reduce(
        (prev: { size: number }, curr: { size: number }) => {
          return prev.size < curr.size ? prev : curr;
        }
      );
      emit("modelChange", smallestModel.name);
    }
  } catch (err) {
    error.value =
      err instanceof Error ? err.message : "Unknown error fetching models";
    console.error("Error fetching models:", err);
  } finally {
    isLoading.value = false;
  }
}

function selectModel(model: string) {
  emit("modelChange", model);
  isDropdownOpen.value = false;
}

function toggleDropdown() {
  isDropdownOpen.value = !isDropdownOpen.value;
}

function clearAllMessages() {
  emit("clearMessages");
}

function handleStreamToggle() {
  streamMessage.value = !streamMessage.value;
  emit("streamToggle", streamMessage.value);
}

onMounted(fetchModels);
</script>

<template>
  <header
    class="fixed top-0 left-0 right-0 z-10 bg-zinc-900/80 backdrop backdrop-blur-xl shadow-md"
  >
    <div class="max-w-6xl mx-auto flex items-center justify-between p-4">
      <h1
        class="text-2xl md:text-4xl font-bold tracking-wider bg-gradient-to-r from-blue-500 to-cyan-400 bg-clip-text text-transparent"
      >
        rovert Chat
      </h1>

      <div class="flex items-center space-x-3">
        <!-- Stream Response Toggle Button -->
        <button
          @click="handleStreamToggle"
          class="flex items-center px-3 py-2 bg-zinc-800 hover:bg-zinc-700 rounded-lg border border-zinc-700 text-white transition-colors"
          :class="{ 'bg-blue-600 hover:bg-blue-700': streamMessage }"
          title="Toggle stream response"
        >
          <span v-if="streamMessage">Streaming</span>
          <span v-else>Instant</span>
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5 ml-2"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              fill-rule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zM3 10a7 7 0 1114 0 7 7 0 01-14 0z"
              clip-rule="evenodd"
            />
          </svg>
        </button>

        <!-- Clear Messages Button -->
        <button
          @click="clearAllMessages"
          class="flex items-center px-3 py-2 bg-zinc-800 hover:bg-zinc-700 rounded-lg border border-zinc-700 text-white transition-colors"
          title="Clear all messages"
        >
          Reset
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5 ml-2"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              fill-rule="evenodd"
              d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
              clip-rule="evenodd"
            />
          </svg>
        </button>

        <!-- Model selector dropdown container with relative positioning -->
        <div class="relative">
          <!-- Model selector dropdown button -->
          <button
            @click="toggleDropdown"
            class="flex items-center space-x-2 px-3 py-2 bg-zinc-800 hover:bg-zinc-700 rounded-lg border border-zinc-700 text-white transition-colors"
            :disabled="isLoading"
          >
            <span v-if="isLoading" class="text-sm md:text-base"
              >Loading models...</span
            >
            <span v-else-if="error" class="text-sm md:text-base text-red-400"
              >Error loading models</span
            >
            <span v-else class="text-sm md:text-base">{{
              currentModel || "Select model"
            }}</span>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-4 w-4"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M19 9l-7 7-7-7"
              />
            </svg>
          </button>

          <!-- Dropdown menu positioned relative to its container -->
          <div
            v-if="isDropdownOpen && !isLoading"
            class="absolute top-full right-0 mt-2 w-60 bg-zinc-800/80 border border-zinc-700 rounded-lg shadow-lg py-1 z-20 backdrop-filter backdrop-blur-md"
          >
            <div v-if="error" class="px-4 py-2 text-red-400">{{ error }}</div>
            <div
              v-else-if="availableModels.length === 0"
              class="px-4 py-2 text-zinc-400"
            >
              No models found
            </div>
            <button
              v-for="model in availableModels"
              :key="model"
              @click="selectModel(model)"
              class="block w-full text-left px-4 py-2 hover:bg-zinc-700 text-white transition-colors"
              :class="{
                'bg-blue-600 hover:bg-blue-700': model === currentModel,
              }"
            >
              {{ model }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>
