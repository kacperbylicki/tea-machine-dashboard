<template>
    <h2>{{ tea?.name }} - Tea Details</h2>
    <button @click="invokeHelloRust()">Hello Rust</button>
    <button @click="redirect()">Back</button>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/tauri";
import teaJson from '@/assets/tea.json'

const route = useRoute();
const router = useRouter();

const redirect = () => {
  router.push("/");
};

const invokeHelloRust = async () => {
  try {
    const response = await invoke("hello_rust")
    console.log(response);
  } catch (error) {
    console.log(error);
  }
};

const getTeaFromSlug = (slug: string | string[]) => {
  return teaData.value.find((tea: any) => tea.slug === slug);
};

const { slug } = route.params;

const teaData = ref(teaJson);
const tea = ref(getTeaFromSlug(slug));
</script>