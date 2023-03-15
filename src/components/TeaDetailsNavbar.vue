<template>
    <button @click="redirectToHome()">Back</button>
</template>
<script setup lang="ts">
import { useRouter } from "vue-router";

const router = useRouter();

const redirectToHome = () => {
  router.push("/");
};
</script>