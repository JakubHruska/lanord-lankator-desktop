<script setup>
import {ref, onMounted, onUnmounted} from "vue";
import api from "@/api/axios.js";
import {useSettingsStore} from "@/stores/settings.js";
import {downloadPackage, extractPackage, saveManifest} from "@/services/downloadService.js";
import {useLibraryStore} from "@/stores/library.js";
import Card from 'primevue/card';
import Button from 'primevue/button';
import Tag from 'primevue/tag';
import ProgressSpinner from 'primevue/progressspinner';
import Message from "primevue/message";
import {useToast} from "primevue";

const settingsStore = useSettingsStore();
const settings = settingsStore.settings;

const TYPE_LABELS = {
  installer:     { label: 'Instalátor',    severity: 'info' },
  ready_to_play: { label: 'Ready to Play', severity: 'success' },
  patch:         { label: 'Patch / Fix',   severity: 'warn' },
  other:         { label: 'Ostatní',       severity: 'secondary' },
};
const getTypeTag = (type) => TYPE_LABELS[type] ?? { label: type, severity: 'secondary' };

/** @type {import('vue').Ref<import('@/types').Package[]>} */
const packages = ref([]);
const loading = ref(true);
let refreshInterval = null;

const toast = useToast();
const downloadingItems = ref(new Set());
const libraryStore = useLibraryStore();

const tooltipFor = (pkg) => {
  if (!settings.downloadPath) return 'Nejprve nastavte složku pro stahování v nastavení';
  if (libraryStore.isInstalled(pkg.slug)) return 'Balíček je již nainstalován';
  return null;
};

const formatSize = (bytes) => {
  if (!bytes) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

const fetchPackages = async () => {
  const loadingTimeout = setTimeout(() => { loading.value = true; }, 200);
  try {
    const response = await api.get('/packages/');
    packages.value = response.data;
  } catch (error) {
    console.error('Chyba při načítání:', error);
  } finally {
    clearTimeout(loadingTimeout);
    loading.value = false;
  }
};

onMounted(() => {
  fetchPackages();
  refreshInterval = setInterval(() => {
    if (settings.autoRefresh) fetchPackages();
  }, 60000);
});

onUnmounted(() => {
  if (refreshInterval) clearInterval(refreshInterval);
});

const handleDownload = async (pkg) => {
  if (libraryStore.isInstalled(pkg.slug)) {
    toast.add({ severity: 'warn', summary: 'Již nainstalováno', detail: `Balíček ${pkg.title} je již nainstalován.`, group: 'tc', life: 4000 });
    return;
  }

  downloadingItems.value.add(pkg.id);
  try {
    toast.add({ severity: 'info', summary: 'Stahování', detail: `Stahuji ${pkg.title}...`, group: 'tc', life: 3000 });
    await downloadPackage(pkg);
    await saveManifest(pkg);
    toast.add({ severity: 'info', summary: 'Rozbalování', detail: `Staženo, rozbaluji ${pkg.title}...`, group: 'tc', life: 3000 });
    const appDir = await extractPackage(pkg);
    await libraryStore.syncWithDisk();
    await libraryStore.setPackageReady(pkg.slug);
    toast.add({ severity: 'success', summary: 'Hotovo', detail: `Balíček stažen do: ${appDir}`, group: 'tc', life: 5000 });
  } catch (error) {
    toast.add({ severity: 'error', summary: 'Chyba', detail: error.message, group: 'tc', life: 5000 });
  } finally {
    downloadingItems.value.delete(pkg.id);
  }
};
</script>

<template>
  <div class="py-0 px-4">
    <Message v-if="!settings.downloadPath" severity="warn" class="mb-4">
      Pozor: V nastavení nemáte vybranou složku pro stahování her.
    </Message>

    <div class="flex justify-content-between align-items-center mb-4">
      <h1 class="m-0">Dostupné balíčky</h1>
      <div class="flex items-center gap-2">
        <Tag :value="`${packages.length} dostupných`" severity="info"/>
        <Button icon="pi pi-refresh" :loading="loading" @click="fetchPackages" label="Obnovit" outlined severity="secondary"/>
      </div>
    </div>

    <div v-if="loading" class="flex justify-content-center align-items-center" style="height: 50vh">
      <ProgressSpinner/>
    </div>

    <div v-else class="grid">
      <div v-for="pkg in packages" :key="pkg.id" class="col-12 md:col-6 lg:col-4 xl:col-3">
        <Card class="h-full flex flex-column overflow-hidden shadow-2 hover:shadow-5 transition-duration-200">
          <template #header>
            <div class="bg-cyan-500 flex align-items-center justify-content-center" style="height: 150px">
              <i class="pi pi-box text-6xl text-primary-900 opacity-90"></i>
            </div>
          </template>

          <template #title>
            <div class="flex justify-content-between align-items-start">
              <span>{{ pkg.title }}</span>
              <Tag :value="getTypeTag(pkg.type).label" :severity="getTypeTag(pkg.type).severity"/>
            </div>
          </template>

          <template #content>
            <p class="m-0 text-secondary line-height-3 h-4rem overflow-hidden text-overflow-ellipsis">
              {{ pkg.description || 'Žádný popis k dispozici.' }}
            </p>
            <div class="mt-3 flex align-items-center gap-2 text-sm text-500">
              <i class="pi pi-database"></i>
              <span>{{ formatSize(pkg.file_size) }}</span>
            </div>
          </template>

          <template #footer>
            <div class="flex gap-2 mt-1">
              <Button
                  :label="downloadingItems.has(pkg.id) ? 'Stahuji...' : 'Stáhnout'"
                  :icon="downloadingItems.has(pkg.id) ? 'pi pi-spin pi-spinner' : 'pi pi-download'"
                  class="flex-grow-1"
                  :disabled="!settings.downloadPath || downloadingItems.has(pkg.id) || libraryStore.isInstalled(pkg.slug)"
                  :title="tooltipFor(pkg) ?? undefined"
                  @click="handleDownload(pkg)"
              />
              <Button icon="pi pi-info-circle" outlined severity="secondary"/>
            </div>
          </template>
        </Card>
      </div>
    </div>

    <div v-if="!loading && packages.length === 0" class="text-center p-8">
      <i class="pi pi-search text-6xl text-400 mb-4"></i>
      <p class="text-xl text-500">Žádné balíčky nebyly nalezeny.</p>
    </div>
  </div>
</template>

<style scoped>

</style>