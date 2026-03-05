import { invoke } from '@tauri-apps/api/core';
import { settings } from '@/stores/settings';

export const downloadPackage = async (pkg) => {
    const downloadPath = settings.value.downloadPath;
    if (!downloadPath) throw new Error("Není nastavena cesta pro instalaci!");

    await invoke('download_package', {
        url: pkg.archive_file,
        downloadPath: downloadPath,
        slug: pkg.slug,
    });
};

export const extractPackage = async (pkg) => {
    const downloadPath = settings.value.downloadPath;
    if (!downloadPath) throw new Error("Není nastavena cesta pro instalaci!");

    return await invoke('extract_package', {
        downloadPath: downloadPath,
        slug: pkg.slug,
    });
};

