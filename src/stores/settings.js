import { defineStore } from 'pinia';
import { ref, watch } from 'vue';
import { load } from '@tauri-apps/plugin-store';

export const useSettingsStore = defineStore('settings', () => {
    const settings = ref({
        downloadPath: '',
        serverUrl: 'http://127.0.0.1:8000',
        autoRefresh: true,
    });

    const initialized = ref(false);
    let _store = null;

    async function init() {
        if (initialized.value) return;
        _store = await load('settings.json', { autoSave: true });
        settings.value = {
            downloadPath: await _store.get('downloadPath') ?? '',
            serverUrl:    await _store.get('serverUrl')    ?? 'http://127.0.0.1:8000',
            autoRefresh:  await _store.get('autoRefresh')  ?? true,
        };
        initialized.value = true;

        watch(settings, async (newVal) => {
            if (!_store) return;
            await _store.set('downloadPath', newVal.downloadPath);
            await _store.set('serverUrl',    newVal.serverUrl);
            await _store.set('autoRefresh',  newVal.autoRefresh);
        }, { deep: true });
    }

    async function save() {
        if (!_store) return;

        await _store.set('downloadPath', settings.value.downloadPath);
        await _store.set('serverUrl', settings.value.serverUrl);
        await _store.set('autoRefresh', settings.value.autoRefresh);

        if (typeof _store.save === 'function') {
            await _store.save();
        }
    }

    return { settings, initialized, init, save };
});

// Zpětná kompatibilita – exportuje reaktivní ref přímo
// (komponenty které importují { settings } budou fungovat přes store.settings)
