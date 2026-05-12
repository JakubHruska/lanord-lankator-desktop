import { createApp } from 'vue';
import { createPinia } from "pinia";
import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura';
import App from './App.vue';
import router from "./router";
import Tooltip from 'primevue/tooltip';
import ToastService from 'primevue/toastservice';
import { useSettingsStore } from '@/stores/settings';

import 'primeicons/primeicons.css';
import 'primeflex/primeflex.css';

async function bootstrap() {
    // Pouze v dev módu – připojení konzole k Tauri loggeru
    if (import.meta.env.DEV) {
        const { attachConsole } = await import('@tauri-apps/plugin-log');
        await attachConsole();
    }

    const app = createApp(App);
    const pinia = createPinia();

    app.use(pinia);
    app.use(router);
    app.use(PrimeVue, {
        theme: {
            preset: Aura,
            options: {
                darkModeSelector: 'light',
            }
        },
        ripple: false
    });
    app.directive('tooltip', Tooltip);
    app.use(ToastService);

    // Načti settings před prvním renderem
    const settingsStore = useSettingsStore();
    await settingsStore.init();

    app.mount('#app');
}

bootstrap();
