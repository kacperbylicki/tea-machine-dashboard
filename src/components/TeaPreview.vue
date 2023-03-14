<template>
    <button>
        <Icon class="previewImage" :iconFileName="tea.iconFileName" />
        <h3>{{ tea.name }}</h3>
    </button>
</template>
  
<script setup lang="ts">
import Icon from '@/components/Icon.vue';
import { toRefs } from "vue";

const props = defineProps({
  tea: {
    type: Object,
    default: () => {},
  },
});

const { tea } = toRefs(props);
</script>

<style scoped>
.previewImage {
  height: 6em;
  padding: 1rem;
  will-change: filter;
  transition: filter 300ms;
}

button {
    padding: 1rem;
    margin: 0 1rem;
}

h3 {
  margin: .5rem auto;
}
</style>