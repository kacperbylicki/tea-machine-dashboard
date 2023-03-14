<template>
    <component :is="icon"/>
  </template>
  
  <script lang="ts" setup>
  import { defineAsyncComponent } from 'vue';

  const { iconFileName } = defineProps({
    iconFileName: {
      type: String,
      required: true
    },
  });
  
  const icon = defineAsyncComponent(() => import(`@/assets/icons/${iconFileName}.svg`));
  </script>