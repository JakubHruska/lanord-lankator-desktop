import axios from 'axios';
import { useSettingsStore } from '@/stores/settings';
import { fetch as tauriFetch } from '@tauri-apps/plugin-http';
import { error as logError, info as logInfo } from '@tauri-apps/plugin-log';

// V produkčním Tauri buildu je window.__TAURI__ dostupné — použijeme Tauri fetch
// aby požadavky nešly přes Webview (který interceptuje URL jako asset)
const customFetch = (typeof window !== 'undefined' && window.__TAURI__)
  ? tauriFetch
  : window.fetch.bind(window);

const api = axios.create({
    baseURL: 'http://127.0.0.1:8000/api/',
    headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
    },
    timeout: 10000,
    adapter: 'fetch',
    fetch: customFetch,
});

// Interceptor: vždy použij aktuální serverUrl ze settings (i po změně v nastavení)
api.interceptors.request.use(
    (config) => {
        const store = useSettingsStore();
        // Odstranění přebytečného koncového lomítka a zajištění, že se nespojí dvojité lomítko
        const base = store.settings.serverUrl.replace(/\/+$/, '');
        config.baseURL = `${base}/api/`;

        // Zabráníme posílání dvojitých lomítek na začátku URL
        if (config.url && config.url.startsWith('/')) {
            config.url = config.url.substring(1);
        }

        logInfo(`API Request: ${config.method.toUpperCase()} ${config.baseURL}${config.url}`);
        return config;
    },
    (error) => {
        logError(`API Request Error: ${error.message}`);
        return Promise.reject(error)
    }
);

// Response interceptor
api.interceptors.response.use(
    (response) => {
        logInfo(`API Response: ${response.config.url} [${response.status}]`);
        return response;
    },
    (error) => {
        const msg = error.response
            ? `API Error: ${error.response.status} na ${error.config?.url} - ${JSON.stringify(error.response.data || {})}`
            : `API Error: ${error.message} na ${error.config?.url}`;

        console.error(msg, error);
        logError(msg);
        return Promise.reject(error);
    }
);

export default api;