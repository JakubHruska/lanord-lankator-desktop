import { ref, watch } from 'vue';
import { load } from '@tauri-apps/plugin-store';

const store = await load('settings.json', { autoSave: true });

export const settings = ref({
    downloadPath: await store.get('downloadPath') ?? '',
    serverUrl: await store.get('serverUrl') ?? 'http://127.0.0.1:8000',
    autoRefresh: await store.get('autoRefresh') ?? true
});

watch(settings, async (newVal) => {
    await store.set('downloadPath', newVal.downloadPath);
    await store.set('serverUrl', newVal.serverUrl);
    await store.set('autoRefresh', newVal.autoRefresh);
}, { deep: true });
