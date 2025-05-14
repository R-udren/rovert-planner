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
const streamMessage = ref(true);
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

    // First try to get the model from localStorage
    const savedModel = localStorage.getItem("selectedModel");

    // If a model is saved and it's in the available models, use it
    if (savedModel && availableModels.value.includes(savedModel)) {
      emit("modelChange", savedModel);
    }
    // If no model is currently selected and no saved model, select the smallest one
    else if (!props.currentModel && availableModels.value.length > 0) {
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
  localStorage.setItem("selectedModel", model);
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

function refetchModels() {
  isLoading.value = true;
  error.value = null;
  fetchModels();
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
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            :class="{ 'text-blue-400': streamMessage }"
          >
            <!-- Stream mode shows wave/flow lines -->
            <g v-if="streamMessage">
              <path d="M2 12h4" />
              <path d="M9 12h4" />
              <path d="M16 12h4" />
              <path d="M4 6h2" />
              <path d="M14 6h4" />
              <path d="M8 18h2" />
              <path d="M14 18h4" />
            </g>
            <!-- Instant mode shows lightning bolt -->
            <g v-else>
              <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" />
            </g>
          </svg>
        </button>

        <!-- Clear Messages Button -->
        <button
          @click="clearAllMessages"
          class="flex items-center px-3 py-2 bg-zinc-800 hover:bg-zinc-700 rounded-lg border border-zinc-700 text-white transition-colors group"
          title="Clear all messages"
        >
          Reset
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5 ml-2 transition-colors duration-150 group-hover:text-red-200 group-active:text-red-400"
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

        <div class="relative">
          <button
            @click="error ? refetchModels() : toggleDropdown()"
            class="flex items-center space-x-2 px-3 py-2 bg-zinc-800 hover:bg-zinc-700 rounded-lg border border-zinc-700 text-white transition-colors"
            :disabled="isLoading"
            :class="{ 'border-red-500': error }"
          >
            <span v-if="isLoading" class="text-sm md:text-base"
              >Loading models...</span
            >
            <span
              v-else-if="error"
              class="text-sm md:text-base text-red-400 flex items-center"
            >
              <span>Try again</span>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-4 w-4 ml-2"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                />
              </svg>
            </span>
            <span v-else class="text-sm md:text-base">{{
              currentModel || "Select model"
            }}</span>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-4 w-4 transition-transform"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              :class="{ 'rotate-90': isDropdownOpen }"
              stroke-width="2"
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
            class="absolute top-full right-0 mt-2 w-60 bg-zinc-800/90 border border-zinc-700 rounded-lg shadow-lg py-1 z-20 backdrop-filter backdrop-blur-xl"
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
              class="block w-full text-left px-4 py-2 hover:bg-zinc-700 text-white transition-colors rounded-md"
              :class="{
                'bg-blue-600 hover:bg-blue-800!': model === currentModel,
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
