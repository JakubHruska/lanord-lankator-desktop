import axios from 'axios';
import { useSettingsStore } from '@/stores/settings';

const api = axios.create({
    baseURL: 'http://127.0.0.1:8000/api/',
    headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
    },
    timeout: 10000,
});

// Interceptor: vždy použij aktuální serverUrl ze settings (i po změně v nastavení)
api.interceptors.request.use(
    (config) => {
        const { settings } = useSettingsStore();
        config.baseURL = `${settings.serverUrl}/api/`;
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