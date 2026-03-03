import { ref, watch } from 'vue';

const STORAGE_KEY = 'lanord_settings';

// Načtení z localStorage při startu
const savedSettings = JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}');

export const settings = ref({
    installPath: savedSettings.installPath || '',
    serverUrl: savedSettings.serverUrl || 'http://127.0.0.1:8000',
    autoRefresh: savedSettings.autoRefresh ?? true
});

// Sledování změn a automatické ukládání
watch(settings, (newVal) => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(newVal));
}, { deep: true });