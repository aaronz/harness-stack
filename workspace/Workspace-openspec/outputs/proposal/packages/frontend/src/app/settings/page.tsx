'use client';

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import DashboardLayout from '@/components/Layout';
import { useAuthStore } from '@/store/auth';
import { providerApi, configApi } from '@/lib/api';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { Settings, Plus, Trash2, Save } from 'lucide-react';

export default function SettingsPage() {
  const router = useRouter();
  const { isAuthenticated, user } = useAuthStore();
  const queryClient = useQueryClient();

  const [activeTab, setActiveTab] = useState<'providers' | 'config'>('providers');

  const { data: providers } = useQuery({
    queryKey: ['providers'],
    queryFn: () => providerApi.list().then(res => res.data),
    enabled: isAuthenticated,
  });

  const { data: config } = useQuery({
    queryKey: ['config'],
    queryFn: () => configApi.get().then(res => res.data),
    enabled: isAuthenticated,
  });

  const createProvider = useMutation({
    mutationFn: providerApi.create,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['providers'] }),
  });

  const deleteProvider = useMutation({
    mutationFn: providerApi.delete,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['providers'] }),
  });

  const updateConfig = useMutation({
    mutationFn: configApi.update,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['config'] }),
  });

  useEffect(() => {
    if (!isAuthenticated) {
      router.push('/login');
    }
  }, [isAuthenticated, router]);

  if (!isAuthenticated) return null;

  const isAdmin = user?.role === 'ADMIN';

  return (
    <DashboardLayout>
      <div className="max-w-6xl">
        <h1 className="text-2xl font-bold mb-6">Settings</h1>

        <div className="flex gap-4 mb-6 border-b border-gray-200">
          <button
            onClick={() => setActiveTab('providers')}
            className={`px-4 py-2 font-medium border-b-2 transition-colors ${
              activeTab === 'providers'
                ? 'border-primary text-primary'
                : 'border-transparent text-gray-500 hover:text-gray-700'
            }`}
          >
            LLM Providers
          </button>
          <button
            onClick={() => setActiveTab('config')}
            className={`px-4 py-2 font-medium border-b-2 transition-colors ${
              activeTab === 'config'
                ? 'border-primary text-primary'
                : 'border-transparent text-gray-500 hover:text-gray-700'
            }`}
          >
            Evaluation Config
          </button>
        </div>

        {activeTab === 'providers' && (
          <div className="bg-white rounded-lg shadow-sm border border-gray-200">
            <div className="p-4 border-b border-gray-200 flex justify-between items-center">
              <h2 className="font-semibold">LLM Providers</h2>
              {isAdmin && (
                <button className="flex items-center gap-2 px-3 py-1.5 bg-primary text-white rounded hover:bg-primary-dark text-sm">
                  <Plus size={16} /> Add Provider
                </button>
              )}
            </div>

            {!providers || providers.length === 0 ? (
              <div className="p-8 text-center text-gray-500">
                No LLM providers configured. Default OpenAI will be used.
              </div>
            ) : (
              <div className="divide-y divide-gray-100">
                {providers.map((provider: any) => (
                  <div key={provider.id} className="p-4 flex items-center justify-between">
                    <div>
                      <div className="font-medium">{provider.name}</div>
                      <div className="text-sm text-gray-500">
                        {provider.provider} - {provider.model}
                      </div>
                    </div>
                    <div className="flex items-center gap-4">
                      <span className={`px-2 py-1 rounded text-xs ${
                        provider.enabled ? 'bg-green-50 text-green-600' : 'bg-gray-50 text-gray-600'
                      }`}>
                        {provider.enabled ? 'Enabled' : 'Disabled'}
                      </span>
                      <span className="text-sm text-gray-500">Priority: {provider.priority}</span>
                      {isAdmin && (
                        <button
                          onClick={() => deleteProvider.mutate(provider.id)}
                          className="p-1 text-red-500 hover:bg-red-50 rounded"
                        >
                          <Trash2 size={16} />
                        </button>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            )}

            <div className="p-4 bg-gray-50 text-sm text-gray-600">
              <strong>Note:</strong> Configure API keys via environment variables: OPENAI_API_KEY, ANTHROPIC_API_KEY, GOOGLE_API_KEY
            </div>
          </div>
        )}

        {activeTab === 'config' && (
          <div className="bg-white rounded-lg shadow-sm border border-gray-200">
            <div className="p-4 border-b border-gray-200">
              <h2 className="font-semibold">Evaluation Configuration</h2>
            </div>

            {config && (
              <div className="p-6 space-y-6">
                <div>
                  <h3 className="font-medium mb-3">Dimension Weights</h3>
                  <div className="grid grid-cols-5 gap-4">
                    {Object.entries(config.dimensionWeights).map(([key, value]: [string, any]) => (
                      <div key={key}>
                        <label className="block text-sm text-gray-600 mb-1 capitalize">
                          {key.replace(/([A-Z])/g, ' $1').trim()}
                        </label>
                        <input
                          type="number"
                          step="0.01"
                          value={value}
                          className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-primary"
                          disabled={!isAdmin}
                        />
                      </div>
                    ))}
                  </div>
                </div>

                <div>
                  <h3 className="font-medium mb-3">Complexity Penalties</h3>
                  <div className="grid grid-cols-3 gap-4">
                    {Object.entries(config.complexityPenalties).map(([key, value]: [string, any]) => (
                      <div key={key}>
                        <label className="block text-sm text-gray-600 mb-1">{key}</label>
                        <input
                          type="number"
                          step="0.1"
                          value={value}
                          className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-primary"
                          disabled={!isAdmin}
                        />
                      </div>
                    ))}
                  </div>
                </div>

                <div>
                  <h3 className="font-medium mb-3">Score Thresholds</h3>
                  <div className="grid grid-cols-3 gap-4">
                    {Object.entries(config.thresholds).map(([key, value]: [string, any]) => (
                      <div key={key}>
                        <label className="block text-sm text-gray-600 mb-1">{key}-Level Min</label>
                        <input
                          type="number"
                          value={value}
                          className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-primary"
                          disabled={!isAdmin}
                        />
                      </div>
                    ))}
                  </div>
                </div>

                {isAdmin && (
                  <button className="flex items-center gap-2 px-4 py-2 bg-primary text-white rounded hover:bg-primary-dark">
                    <Save size={16} /> Save Configuration
                  </button>
                )}
              </div>
            )}
          </div>
        )}
      </div>
    </DashboardLayout>
  );
}