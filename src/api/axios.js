import axios from 'axios';
import { settings } from '@/stores/settings';

const api = axios.create({
    baseURL: `${settings.value.serverUrl}/api/`,
    headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
    },
    timeout: 10000,
});

// Interceptor: vždy použij aktuální serverUrl ze settings (i po změně v nastavení)
api.interceptors.request.use(
    (config) => {
        config.baseURL = `${settings.value.serverUrl}/api/`;
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