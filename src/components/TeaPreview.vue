<template>
    <button>
        <Icon class="previewImage" :iconFileName="tea.iconFileName" />
        <h3>{{ tea.name }}</h3>
    </button>
</template>
  
<script setup lang="ts">
import { toRefs } from "vue";
import Icon from '@/components/Icon.vue';

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
    margin-bottom: 1rem;
}

button:not(:last-child) {
  padding: 1rem;
  margin: auto 1rem 1rem auto;
}

h3 {
  margin: .5rem auto;
}
</style>