import { createApp } from 'vue';
import { createPinia } from "pinia";
import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura';
import App from './App.vue';
import router from "./router";
import Tooltip from 'primevue/tooltip';

import 'primeicons/primeicons.css';
import 'primeflex/primeflex.css';

const app = createApp(App);

app.use(router)
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

app.mount('#app');