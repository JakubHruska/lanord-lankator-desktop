<script setup>
import { onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useToast } from 'primevue';
import { useLibraryStore } from '@/stores/library.js';
import { settings } from '@/stores/settings.js';
import Card from 'primevue/card';
import Button from 'primevue/button';
import Tag from 'primevue/tag';
import ProgressSpinner from 'primevue/progressspinner';
import Message from 'primevue/message';

const libraryStore = useLibraryStore();
const toast = useToast();

const TYPE_LABELS = {
  installer:     { label: 'Instalátor',   severity: 'info' },
  ready_to_play: { label: 'Ready to Play', severity: 'success' },
  patch:         { label: 'Patch / Fix',   severity: 'warn' },
  other:         { label: 'Ostatní',       severity: 'secondary' },
};
const getTypeTag = (type) => TYPE_LABELS[type] ?? { label: type, severity: 'secondary' };

const isReady = (pkg) => pkg.status === 'ready';

const formatSize = (bytes) => {
  if (!bytes) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

const openFolder = async (pkg) => {
  try {
    await invoke('open_folder', { packagePath: pkg.package_path });
  } catch (e) {
    toast.add({ severity: 'error', summary: 'Chyba', detail: e, group: 'tc', life: 5000 });
  }
};

const uninstall = async (pkg) => {
  try {
    await invoke('uninstall_package', {
      downloadPath: settings.value.downloadPath,
      slug: pkg.slug,
    });
    libraryStore.removePackage(pkg.slug);
    toast.add({
      severity: 'success',
      summary: 'Odinstalováno',
      detail: `${pkg.title} byl odstraněn.`,
      group: 'tc',
      life: 4000,
    });
  } catch (e) {
    toast.add({ severity: 'error', summary: 'Chyba při odinstalaci', detail: e, group: 'tc', life: 5000 });
  }
};

onMounted(() => {
  libraryStore.syncWithDisk();
});
</script>

<template>
  <div class="py-0 px-4">
    <Message v-if="!settings.downloadPath" severity="warn" class="mb-4">
      Pozor: V nastavení nemáte vybranou složku pro stahování her.
    </Message>

    <div class="flex justify-content-between align-items-center mb-4">
      <h1 class="m-0">Stažené balíčky</h1>
      <Button
        label="Obnovit"
        icon="pi pi-refresh"
        severity="secondary"
        :loading="libraryStore.loading"
        @click="libraryStore.syncWithDisk()"
      />
    </div>

    <Message v-if="libraryStore.error" severity="error" class="mb-4">
      {{ libraryStore.error }}
    </Message>

    <div v-if="libraryStore.loading" class="flex justify-content-center mt-6">
      <ProgressSpinner />
    </div>

    <div v-else-if="libraryStore.packageList.length === 0" class="text-center text-color-secondary mt-6">
      <i class="pi pi-inbox text-5xl mb-3 block" />
      <p>Žádné nainstalované balíčky.</p>
    </div>

    <div v-else class="grid">
      <div
        v-for="pkg in libraryStore.packageList"
        :key="pkg.slug"
        class="col-12 md:col-6 lg:col-4 xl:col-3"
      >
        <Card
          class="h-full"
          :class="{ 'pkg-installing': !isReady(pkg) }"
        >
          <template #title>
            <div class="flex justify-content-between gap-2 align-items-center">
              <span>{{ pkg.title }}</span>
              <div class="flex gap-1">
                <Tag
                  v-if="!isReady(pkg)"
                  value="Stahuji..."
                  severity="warn"
                  icon="pi pi-spin pi-spinner"
                />
                <Tag :value="getTypeTag(pkg.type).label" :severity="getTypeTag(pkg.type).severity" />
              </div>
            </div>
          </template>

          <template #subtitle>
            <span class="text-xs text-color-secondary font-mono">{{ pkg.slug }}</span>
          </template>

          <template #content>
            <p class="text-sm m-0 mb-2 line-clamp-3">{{ pkg.description }}</p>
            <div class="flex gap-3 text-xs text-color-secondary">
              <span><i class="pi pi-database mr-1" />{{ formatSize(pkg.file_size) }}</span>
              <span v-if="pkg.created_at">
                <i class="pi pi-calendar mr-1" />{{ new Date(pkg.created_at).toLocaleDateString('cs-CZ') }}
              </span>
            </div>
          </template>

          <template #footer>
            <div class="flex gap-2">
              <!-- Připravený balíček -->
              <template v-if="isReady(pkg)">
                <Button
                  label="Otevřít složku"
                  icon="pi pi-folder-open"
                  severity="info"
                  size="small"
                  class="flex-1"
                  @click="openFolder(pkg)"
                />
                <Button
                  label="Odstranit"
                  icon="pi pi-trash"
                  severity="danger"
                  size="small"
                  class="flex-1"
                  @click="uninstall(pkg)"
                />
              </template>

              <!-- Instalující se balíček -->
              <template v-else>
                <Button
                  label="Stahuji..."
                  icon="pi pi-spin pi-spinner"
                  severity="secondary"
                  size="small"
                  class="flex-1"
                  disabled
                />
              </template>
            </div>
          </template>
        </Card>
      </div>
    </div>
  </div>
</template>

<style scoped>
.pkg-installing {
  filter: grayscale(100%) opacity(0.7);
  pointer-events: none;
}
</style>

