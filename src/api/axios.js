import axios from 'axios';

const api = axios.create({
    baseURL: 'http://127.0.0.1:8000/api/',
    headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
    },
    timeout: 10000,
});

// Request interceptor
api.interceptors.request.use(
    (config) => {
        // Zde lze přidat token např.:
        // const token = localStorage.getItem('token');
        // if (token) config.headers.Authorization = `Bearer ${token}`;
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