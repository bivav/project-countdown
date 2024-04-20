<template>
    <div class="countdown">
        <p>Countdown to July 5, 2024, 1:00 PM:</p>
        <p>{{ remainingTime }}</p>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const remainingTime = ref("");

async function updateCountdown() {
    try {
        const result = await invoke("countdown");
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
</script>

<style scoped>
.countdown {
    font-family: "Arial", sans-serif;
    text-align: center;
}
</style>
