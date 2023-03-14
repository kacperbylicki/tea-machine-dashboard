<template>
    <TeaPreview v-for="tea in teaData" :key="tea.id" :tea="tea" @click="redirectToTeaDetails(tea.slug)" />
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import teaJson from '@/assets/tea.json'
import TeaPreview from '@/components/TeaPreview.vue';

const router = useRouter();

const redirectToTeaDetails = (teaSlug: string) => {
  router.push(`/tea/${teaSlug}`);
};

const teaData = ref(teaJson);
</script>