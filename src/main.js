import { createApp } from 'vue';
import { createPinia } from "pinia";
import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura';
import App from './App.vue';
import router from "./router";

import 'primeicons/primeicons.css';

const app = createApp(App);

app.use(router)
app.use(PrimeVue, {
    theme: {
        preset: Aura,
        options: {
            darkModeSelector: 'system', // Automaticky přepne na dark mode podle Windows/macOS
        }
    },
    ripple: false
});

app.mount('#app');