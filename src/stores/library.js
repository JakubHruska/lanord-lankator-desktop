import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { settings } from '@/stores/settings';

export const useLibraryStore = defineStore('library', () => {
    /** @type {import('vue').Ref<Map<string, import('@/types').InstalledPackage>>} */
    const installedPackages = ref(new Map());
    const loading = ref(false);
    const error = ref(null);

    /** Pole balíčků pro iteraci v šablonách */
    const packageList = computed(() => Array.from(installedPackages.value.values()));

    /** Zkontroluje, zda je slug nainstalován */
    const isInstalled = (slug) => installedPackages.value.has(slug);

    /** Synchronizuje stav s diskem přes Rust scan_library */
    async function syncWithDisk() {
        const downloadPath = settings.value.downloadPath;
        if (!downloadPath) {
            error.value = 'Není nastavena cesta pro instalaci.';
            return;
        }

        loading.value = true;
        error.value = null;

        try {
            const packages = await invoke('scan_library', { downloadPath });
            const map = new Map();
            for (const pkg of packages) {
                map.set(pkg.slug, pkg);
            }
            installedPackages.value = map;
        } catch (e) {
            error.value = e?.message ?? String(e);
        } finally {
            loading.value = false;
        }
    }

    /** Odebere balíček ze store po odinstalaci */
    function removePackage(slug) {
        installedPackages.value.delete(slug);
    }

    /**
     * Přepne status balíčku na "ready" – v paměti i na disku.
     * @param {string} slug
     */
    async function setPackageReady(slug) {
        const pkg = installedPackages.value.get(slug);
        if (!pkg) return;

        await invoke('update_manifest_status', {
            packagePath: pkg.package_path,
            status: 'ready',
        });

        // Optimistická aktualizace v paměti
        installedPackages.value.set(slug, { ...pkg, status: 'ready' });
    }

    return {
        installedPackages,
        packageList,
        loading,
        error,
        isInstalled,
        syncWithDisk,
        removePackage,
        setPackageReady,
    };
});
