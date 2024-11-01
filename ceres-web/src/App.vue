<script setup lang="ts">
import {useQuery} from "@tanstack/vue-query";
import axios from "axios";
import {Service} from "./types.ts";

const getServices = (): Promise<Group[]> =>
    axios.get("https://api.status.icaruspw.dev/services").then((response) => response.data as Map<string, Service>)

const {data, isSuccess} = useQuery({queryKey: ["services"], queryFn: getServices})
</script>

<template>
  <div class="md:bg-amber-100 h-screen w-screen flex flex-col">
    <div class="flex-grow flex justify-center items-center h-full">
      <div class="bg-white h-full w-full md:h-[500px] md:w-[500px] md:rounded-2xl p-6">
        <h1 class="text-2xl">Service Uptime</h1>

        <div v-if="isSuccess">
          {{data}}
        </div>
      </div>
    </div>
    <div class="text-center text-gray-500 m-2">❤ Created by Icarus Project</div>
  </div>
</template>

<style scoped>
</style>
