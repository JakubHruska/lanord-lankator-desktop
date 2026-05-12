<script setup>
import {ref, onMounted} from "vue";
import {useRouter, useRoute} from 'vue-router';
import Tabs from "primevue/tabs";
import TabList from "primevue/tablist";
import Tab from "primevue/tab";
import Toast from 'primevue/toast';
import { useLibraryStore } from '@/stores/library.js';

const router = useRouter();
const route = useRoute();

const libraryStore = useLibraryStore();

onMounted(() => {
  // Sync library on application start so library tab shows data even if user doesn't open it
  libraryStore.syncWithDisk();
});

const onTabChange = (newPath) => {
  router.push(newPath);
};

const items = ref([
  {route: "/store", label: "Balíčky", icon: "pi pi-shopping-cart"},
  {route: "/library", label: "Knihovna", icon: "pi pi-book"},
  {route: "/settings", label: "Nastavení", icon: "pi pi-cog"},
]);

</script>

<template>
  <div>
    <div class="sticky top-0 z-5">
      <Tabs :value="route.path" @update:value="onTabChange">
        <TabList>
          <Tab v-for="item in items" :key="item.route" :value="item.route">
            <div class="flex items-center gap-2">
              <i :class="item.icon"/>
              <span>{{ item.label }}</span>
            </div>
          </Tab>
        </TabList>
      </Tabs>
    </div>

    <Toast class="pt-4" position="top-center" group="tc"/>

    <div class="px-4 py-2">
      <RouterView/>
    </div>
  </div>
</template>

<style scoped>
.tab-link {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: inherit;
  text-decoration: none;
}
</style>

<style>
html, body {
  margin: 0;
  padding: 0;
  font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  /*overflow-y: auto;*/
  scrollbar-gutter: stable; /* reservuje místo pro scrollbar vždy, aby layout neskákal */
}

#app,
.p-component {
  font-family: inherit;
}
</style>
