<script setup>
import {ref, computed} from "vue";
import {useRoute} from "vue-router";
import Tabs from "primevue/tabs";
import TabList from "primevue/tablist";
import Tab from "primevue/tab";

const route = useRoute();

const items = ref([
  {route: "/store", label: "Obchod", icon: "pi pi-shopping-cart"},
  {route: "/library", label: "Knihovna", icon: "pi pi-book"},
]);

const activeRoute = computed(() => route.path);
</script>

<template>
  <Tabs :value="activeRoute">
    <TabList>
      <Tab v-for="item in items" :key="item.route" :value="item.route">
        <router-link v-slot="{ href, navigate }" :to="item.route" custom>
          <a :href="href" @click="navigate" class="tab-link">
            <i :class="item.icon"/>
            <span>{{ item.label }}</span>
          </a>
        </router-link>
      </Tab>
    </TabList>
  </Tabs>
  <RouterView/>
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
  overflow-y: auto;
  //scrollbar-gutter: stable; /* reservuje místo pro scrollbar vždy, aby layout neskákal */
}
</style>
