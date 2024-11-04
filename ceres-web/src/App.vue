<script setup lang="ts">
import {useQuery} from "@tanstack/vue-query";
import axios from "axios";
import {Service} from "./types.ts";
import Loading from "./components/Loading.vue";

const getServices = (): Promise<Map<string, Array<Service>>> =>
    axios.get("https://api.status.icaruspw.dev/services").then((response) => response.data)

const {data, isSuccess} = useQuery({queryKey: ["services"], queryFn: getServices})

const groupBy = (array: Array<Service>) => {
  let newArray: Array<Service> = [];

  if (array.length < 60) {
    newArray = Array.from(
        {length: 60},
        (_) => ({code: 0, time: 0, status: "unknown"})
    )

    for (let i = 0; i < array.length; i++) {
      newArray[i] = array[i];
    }
  } else {
    for (let i = 0; i < array.length; i++) {
      newArray[i] = array[i];
    }
  }

  return newArray.reduce((r, _, i) => {
    if (i % 20 === 0) {
      r.push(newArray.slice(i, i + 20));
    }
    return r;
  }, [] as Array<Service>[]);
}
</script>

<template>
  <div class="md:bg-amber-100 h-screen w-screen flex flex-col">
    <div class="flex-grow flex justify-center items-center h-full">
      <div class="bg-white h-full w-full md:h-[600px] md:w-[450px] md:rounded-2xl flex flex-col justify-center items-center overflow-scroll">
        <br>
        <h1 class="text-2xl">Service Uptime</h1>
        <div class="flex flex-col justify-center items-center">
          <p class="text-sm text-gray-500">A check is performed every minute.</p>
          <p class="text-sm text-gray-500">If the color block is green, the service is available.</p>
          <p class="text-sm text-gray-500">if it is yellow, it is timed out.</p>
        </div>

        <div v-if="!isSuccess">
          <Loading/>
        </div>
        <div v-if="isSuccess">
          <div v-for="(services, key) in data" :key="key" class="my-2">
            <h3 class="text-xl my-2">{{ key }}</h3>
            <div class="flex flex-col gap-1">
              <div v-for="(status, index) in groupBy(services as unknown as Array<Service>)" :key="index">
                <div class="flex flex-row gap-1">
                  <div v-for="(service, index) in status" :key="index">
                    <div v-show="service.status == 'success'"
                         class="bg-green-500 h-[26px] w-[16px] rounded-full"></div>
                    <div v-show="service.status == 'timeout'"
                         class="bg-yellow-300 h-[26px] w-[16px] rounded-full"></div>
                    <div v-show="service.status == 'failure'"
                         class="bg-red-500 h-[26px] w-[16px] rounded-full"></div>
                    <div v-show="service.status == 'unknown'" class="bg-gray-300 h-[26px] w-[16px] rounded-full"></div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <a class="text-center text-gray-500 py-4" href="https://github.com/hanbings/icarus">❤ Created by Icarus Project</a>
  </div>
</template>

<style scoped>
</style>
