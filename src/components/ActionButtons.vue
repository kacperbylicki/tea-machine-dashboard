<template>
    <div>
      <button @click="emit('writeSerial')">Write Serial</button>
      <button @click="emit('startListening')">Start Listening</button>
    </div>
</template>
  
<script setup lang="ts">
const emit = defineEmits(["writeSerial", "startListening"])
</script>