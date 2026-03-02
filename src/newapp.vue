<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Card from "primevue/card";
import InputText from "primevue/inputtext";
import Button from "primevue/button";
import Message from "primevue/message";
import Divider from "primevue/divider";

const greetMsg = ref("");
const name = ref("");
const loading = ref(false);

async function greet() {
  loading.value = true;
  greetMsg.value = await invoke("greet", { name: name.value });
  loading.value = false;
}
</script>

<template>
  <div class="app-container">
    <Card class="main-card">
      <template #header>
        <div class="logo-row">
          <a href="https://vite.dev" target="_blank">
            <img src="/vite.svg" class="logo vite" alt="Vite logo" />
          </a>
          <a href="https://tauri.app" target="_blank">
            <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
          </a>
          <a href="https://vuejs.org/" target="_blank">
            <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
          </a>
        </div>
      </template>

      <template #title>Welcome to Tauri + Vue + PrimeVue</template>

      <template #subtitle>
        Click on the Tauri, Vite, and Vue logos to learn more.
      </template>

      <template #content>
        <Divider />
        <form class="greet-form" @submit.prevent="greet">
          <div class="p-inputgroup">
            <InputText
              id="greet-input"
              v-model="name"
              placeholder="Zadej jméno..."
            />
            <Button
              type="submit"
              label="Pozdravit"
              icon="pi pi-send"
              :loading="loading"
            />
          </div>
        </form>

        <Message v-if="greetMsg" severity="success" class="greet-message" :closable="false">
          {{ greetMsg }}
        </Message>
      </template>
    </Card>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  padding: 2rem;
  box-sizing: border-box;
}

.main-card {
  width: 100%;
  max-width: 500px;
  text-align: center;
}

.logo-row {
  display: flex;
  justify-content: center;
  padding: 1.5rem 1.5rem 0;
  gap: 0.5rem;
}

.logo {
  height: 5em;
  padding: 1em;
  will-change: filter;
  transition: filter 0.75s;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.greet-form {
  display: flex;
  justify-content: center;
  margin-top: 1rem;
}

.p-inputgroup {
  width: 100%;
  max-width: 380px;
}

.greet-message {
  margin-top: 1.2rem;
}
</style>
