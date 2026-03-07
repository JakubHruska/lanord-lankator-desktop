import { invoke } from '@tauri-apps/api/core';
import { useSettingsStore } from '@/stores/settings';

export const downloadPackage = async (pkg) => {
    const { settings } = useSettingsStore();
    const downloadPath = settings.downloadPath;
    if (!downloadPath) throw new Error("Není nastavena cesta pro instalaci!");

    await invoke('download_package', {
        url: pkg.archive_file,
        downloadPath: downloadPath,
        slug: pkg.slug,
    });
};

export const saveManifest = async (pkg) => {
    const { settings } = useSettingsStore();
    const downloadPath = settings.downloadPath;
    if (!downloadPath) throw new Error("Není nastavena cesta pro instalaci!");

    await invoke('save_manifest', {
        downloadPath: downloadPath,
        slug: pkg.slug,
        manifest: pkg.manifest_data,
    });
};

export const extractPackage = async (pkg) => {
    const { settings } = useSettingsStore();
    const downloadPath = settings.downloadPath;
    if (!downloadPath) throw new Error("Není nastavena cesta pro instalaci!");

    return await invoke('extract_package', {
        downloadPath: downloadPath,
        slug: pkg.slug,
    });
};
