import { createApp } from 'vue';
import { createPinia } from "pinia";
import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura';
import App from './App.vue';
import router from "./router";
import Tooltip from 'primevue/tooltip';
import ToastService from 'primevue/toastservice';

import 'primeicons/primeicons.css';
import 'primeflex/primeflex.css';

import { attachConsole } from '@tauri-apps/plugin-log';
const detach = await attachConsole();
// TODO: call detach() if you do not want to print logs to the console anymore

const app = createApp(App);

app.use(router)
app.use(createPinia())
app.use(PrimeVue, {
    theme: {
        preset: Aura,
        options: {
            darkModeSelector: 'light', // TODO: nastavit na system, umožnit přepínání
        }
    },
    ripple: false
});
app.directive('tooltip', Tooltip);
app.use(ToastService);

app.mount('#app');