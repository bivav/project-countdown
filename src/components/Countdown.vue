<template>
    <div class="countdown">
        <input
            type="datetime-local"
            v-model="targetDate"
            @change="updateCountdown"
            @click="closeDateSelectionBox"
        />
        <p>
            Countdown to <span>{{ formattedTargetDate }}</span>
        </p>
        <p>{{ remainingTime }}</p>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const defaultDateValue = "2024-07-05T13:00"; // Default date value in ISO 8601 format
const targetDate = ref(defaultDateValue); // Store the target date
const remainingTime = ref(""); // Store the remaining time

const formattedTargetDate = computed(() => {
    const date = new Date(targetDate.value);
    return date.toLocaleString("en-US", {
        month: "long",
        day: "numeric",
        year: "numeric",
        hour: "numeric",
        minute: "numeric",
        hour12: true,
    });
});

async function updateCountdown() {
    try {
        // get the date from the input here
        const selectedDate = new Date(targetDate.value);
        const dateString = selectedDate.toISOString();

        const result = await invoke("countdown", { targetDate: dateString });
        //@ts-ignore
        remainingTime.value = result;
    } catch (e) {
        console.error("Error fetching countdown:", e);
    }
}

let interval = setInterval(updateCountdown, 1000);

onMounted(() => {
    updateCountdown();
});

onUnmounted(() => {
    clearInterval(interval);
});

function closeDateSelectionBox(this: HTMLInputElement) {
    this.blur();
}
</script>

<style scoped>
.countdown {
    font-family: "Arial", sans-serif;
    text-align: center;
}
</style>
