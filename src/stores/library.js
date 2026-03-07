import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useSettingsStore } from '@/stores/settings';

export const useLibraryStore = defineStore('library', () => {
    /** @type {import('vue').Ref<Map<string, any>>} */
    const installedPackages = ref(new Map());
    const loading = ref(false);
    const error = ref(null);

    /**
     * Sety slugů právě stahovaných balíčků — přežije přepnutí záložky.
     * @type {import('vue').Ref<Set<string>>}
     */
    const downloadingItems = ref(new Set());

    /** Pole balíčků pro iteraci v šablonách */
    const packageList = computed(() => Array.from(installedPackages.value.values()));

    /** Zkontroluje, zda je slug nainstalován */
    const isInstalled = (slug) => installedPackages.value.has(slug);

    /** Zkontroluje, zda se slug právě stahuje */
    const isDownloading = (slug) => downloadingItems.value.has(slug);

    /** Označí balíček jako stahovaný a ihned přidá "installing" záznam do knihovny */
    function startDownload(pkg) {
        downloadingItems.value.add(pkg.slug);
        // Optimisticky zobrazit šedou kartičku hned
        if (!installedPackages.value.has(pkg.slug)) {
            const settingsStore = useSettingsStore();
            installedPackages.value.set(pkg.slug, {
                slug: pkg.slug,
                title: pkg.title,
                description: pkg.description ?? '',
                type: pkg.type,
                file_size: pkg.file_size,
                archive_file: pkg.archive_file ?? '',
                created_at: pkg.created_at ?? null,
                status: 'installing',
                package_path: `${settingsStore.settings.downloadPath}/${pkg.slug}`,
            });
        }
    }

    /** Označí balíček jako hotový po stažení */
    function finishDownload(slug) {
        downloadingItems.value.delete(slug);
    }

    /** Zruší stahování (při chybě) a odebere optimistický záznam pokud status stále installing */
    function cancelDownload(slug) {
        downloadingItems.value.delete(slug);
        const pkg = installedPackages.value.get(slug);
        if (pkg?.status === 'installing') {
            installedPackages.value.delete(slug);
        }
    }

    /** Synchronizuje stav s diskem přes Rust scan_library */
    async function syncWithDisk() {
        const settingsStore = useSettingsStore();
        const downloadPath = settingsStore.settings.downloadPath;
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
            // Zachovat optimistické záznamy pro právě stahované balíčky
            for (const slug of downloadingItems.value) {
                if (!map.has(slug) && installedPackages.value.has(slug)) {
                    map.set(slug, installedPackages.value.get(slug));
                }
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
        downloadingItems,
        isInstalled,
        isDownloading,
        startDownload,
        finishDownload,
        cancelDownload,
        syncWithDisk,
        removePackage,
        setPackageReady,
    };
});
