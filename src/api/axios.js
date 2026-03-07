import axios from 'axios';
import { useSettingsStore } from '@/stores/settings';
import { fetch as tauriFetch } from '@tauri-apps/plugin-http';

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
        config.baseURL = `${store.settings.serverUrl}/api/`;
        return config;
    },
    (error) => Promise.reject(error)
);

// Response interceptor
api.interceptors.response.use(
    (response) => response,
    (error) => {
        console.error('API Error:', error.response?.status, error.message);
        return Promise.reject(error);
    }
);

export default api;