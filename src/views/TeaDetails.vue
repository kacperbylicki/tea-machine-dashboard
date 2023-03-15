<template>
  <TeaDetailsNavbar />
  <h2>{{ tea?.name }} - Tea Details</h2>
  <ActionButtons
    @writeSerial="invokeWriteSerial"
    @startListening="invokeStartAction"
  />
  <SerialResponsesList :serialResponses="serialResponses" />
</template>

<script setup lang="ts">
import ActionButtons from "@/components/ActionButtons.vue";
import SerialResponsesList from "@/components/SerialResponsesList.vue";
import TeaDetailsNavbar from '@/components/TeaDetailsNavbar.vue';
import teaJson from '@/assets/tea.json'
import { SerialResponse } from "@/types/serial-response.type";
import { getTeaFromSlug } from "@/utils/get-tea-from-slug.helper";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { ref } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const { slug } = route.params;

const teaData = ref(teaJson);
const tea = ref(getTeaFromSlug(teaData, slug));
const serialResponses = ref<SerialResponse[]>([]);

const invokeWriteSerial = async () => {
  try {
    const response = await invoke("write_serial", { message: 'Hello!' })
    console.log(response);
  } catch (error) {
    console.log(error);
  }
};

const invokeStartAction = async () => {
  try {
    await invoke("start_action")
  } catch (error) {
    console.log(error);
  }
};

await listen('read_serial', (event) => {
  console.log(event)
  let input = event.payload
  serialResponses.value.push({ timestamp: Date.now(), message: input })
});
</script>