<template>
    <ol>
      <li v-for="serialResponse in serialResponses" :key="serialResponse.timestamp">
        {{ serialResponse.message }}
      </li>
    </ol>
</template>
  
<script setup lang="ts">
import { SerialResponse } from "@/types/serial-response.type";
  
const { serialResponses } = defineProps({
    serialResponses: {
        type: Array as () => SerialResponse[],
        required: true,
    },
});    
</script>