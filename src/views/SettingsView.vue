<script setup>
import { useSettingsStore } from "@/stores/settings.js";
import { open } from '@tauri-apps/plugin-dialog';
import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import Message from 'primevue/message';

const settingsStore = useSettingsStore();
const settings = settingsStore.settings;

const selectDirectory = async () => {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'Vyberte složku pro instalaci her'
  });

  if (selected) {
    settings.downloadPath = selected;
  }
};
</script>

<template>
  <div class="py-0 px-4 flex flex-column gap-4 max-w-90rem">
    <h1 class="m-0">Nastavení</h1>

    <div class="flex flex-column gap-2">
      <label for="server-url" class="font-bold">Adresa package serveru</label>
      <InputText id="server-url" v-model="settings.serverUrl" placeholder="http://..." />
    </div>

    <div class="flex flex-column gap-2">
      <label class="font-bold">Cílová složka pro stahování</label>
      <div class="flex gap-2">
        <InputText v-model="settings.downloadPath" readonly class="flex-grow-1" placeholder="Není vybráno" />
        <Button icon="pi pi-folder-open" @click="selectDirectory" />
      </div>
      <Message v-if="!settings.downloadPath" severity="error" :closable="false">
        Bez vybraného adresáře nebude možné stahovat balíčky!
      </Message>
    </div>
  </div>
</template>