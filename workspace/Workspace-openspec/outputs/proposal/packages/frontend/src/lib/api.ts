import axios from 'axios';

const API_BASE = process.env.NEXT_PUBLIC_API_URL || '/api';

export const api = axios.create({
  baseURL: API_BASE,
  headers: {
    'Content-Type': 'application/json',
  },
});

api.interceptors.request.use((config) => {
  if (typeof window !== 'undefined') {
    const token = localStorage.getItem('token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
  }
  return config;
});

api.interceptors.response.use(
  (response) => response,
  async (error) => {
    if (error.response?.status === 401) {
      const refreshToken = localStorage.getItem('refreshToken');
      if (refreshToken) {
        try {
          const { data } = await axios.post(`${API_BASE}/auth/refresh`, { refreshToken });
          localStorage.setItem('token', data.token);
          localStorage.setItem('refreshToken', data.refreshToken);
          error.config.headers.Authorization = `Bearer ${data.token}`;
          return api.request(error.config);
        } catch {
          localStorage.removeItem('token');
          localStorage.removeItem('refreshToken');
          window.location.href = '/login';
        }
      }
    }
    return Promise.reject(error);
  }
);

export const authApi = {
  register: (email: string, password: string, name?: string) =>
    api.post('/auth/register', { email, password, name }),
  login: (email: string, password: string) =>
    api.post('/auth/login', { email, password }),
  refresh: (refreshToken: string) =>
    api.post('/auth/refresh', { refreshToken }),
};

export const evaluationApi = {
  create: (requirementText: string, format?: string, complexity?: string) =>
    api.post('/evaluations', { requirementText, requirementFormat: format, complexity }),
  get: (id: string) => api.get(`/evaluations/${id}`),
  list: (page = 1, limit = 20) => api.get(`/evaluations?page=${page}&limit=${limit}`),
  batch: (requirements: { text: string; format?: string; complexity?: string }[]) =>
    api.post('/evaluations/batch', { requirements }),
};

export const providerApi = {
  list: () => api.get('/providers'),
  create: (data: any) => api.post('/providers', data),
  update: (id: string, data: any) => api.put(`/providers/${id}`, data),
  delete: (id: string) => api.delete(`/providers/${id}`),
};

export const configApi = {
  get: () => api.get('/config'),
  update: (data: any) => api.put('/config', data),
};