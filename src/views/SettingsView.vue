<script setup>
import { useSettingsStore } from "@/stores/settings.js";
import { open } from '@tauri-apps/plugin-dialog';
import { useToast } from 'primevue';
import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import Message from 'primevue/message';

const settingsStore = useSettingsStore();
const settings = settingsStore.settings;
const toast = useToast();

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

const saveSettings = async () => {
  try {
    await settingsStore.save();
    toast.add({
      severity: 'success',
      summary: 'Uloženo',
      detail: 'Nastavení bylo úspěšně uloženo.',
      group: 'tc',
      life: 3000,
    });
  } catch (error) {
    toast.add({
      severity: 'error',
      summary: 'Chyba',
      detail: error?.message ?? 'Nepodařilo se uložit nastavení.',
      group: 'tc',
      life: 5000,
    });
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

    <div class="flex justify-content-end pt-2">
      <Button label="Uložit" icon="pi pi-check" @click="saveSettings" />
    </div>
  </div>
</template>