import { useState, useEffect } from 'react';

interface Provider {
  id: string;
  name: string;
  model: string;
  config: Record<string, string>;
  isDefault: boolean;
  isLocal: boolean;
}

interface ProviderSelectorProps {
  onProviderChange?: (provider: Provider) => void;
}

export default function ProviderSelector({ onProviderChange }: ProviderSelectorProps) {
  const [providers, setProviders] = useState<Provider[]>([]);
  const [selectedProvider, setSelectedProvider] = useState<string>('');
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchProviders();
  }, []);

  const fetchProviders = async () => {
    try {
      const response = await fetch('/api/config/providers');
      const data = await response.json();
      setProviders(data.providers || []);
      
      const defaultProvider = data.providers?.find((p: Provider) => p.isDefault);
      if (defaultProvider) {
        setSelectedProvider(defaultProvider.id);
      }
    } catch (err) {
      setError('Failed to load providers');
    } finally {
      setIsLoading(false);
    }
  };

  const handleProviderChange = async (providerId: string) => {
    setSelectedProvider(providerId);
    const provider = providers.find((p) => p.id === providerId);
    if (provider && onProviderChange) {
      onProviderChange(provider);
    }

    try {
      await fetch(`/api/config/providers/${providerId}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ isDefault: true }),
      });
    } catch (err) {
      console.error('Failed to set default provider');
    }
  };

  const getProviderIcon = (name: string) => {
    const icons: Record<string, string> = {
      openai: '🤖',
      anthropic: '🧠',
      gemini: '✨',
      ollama: '🏠',
    };
    return icons[name] || '📦';
  };

  if (isLoading) {
    return <div className="p-4 text-gray-500">Loading providers...</div>;
  }

  return (
    <div className="p-4 bg-white rounded-lg border border-gray-200">
      <h3 className="text-lg font-semibold mb-3">LLM Provider</h3>
      
      {error && (
        <div className="mb-3 p-2 bg-red-50 text-red-600 rounded">{error}</div>
      )}

      <div className="space-y-2">
        {providers.map((provider) => (
          <label
            key={provider.id}
            className={`flex items-center p-3 rounded-lg border-2 cursor-pointer transition-colors ${
              selectedProvider === provider.id
                ? 'border-blue-500 bg-blue-50'
                : 'border-gray-200 hover:border-gray-300'
            }`}
          >
            <input
              type="radio"
              name="provider"
              value={provider.id}
              checked={selectedProvider === provider.id}
              onChange={() => handleProviderChange(provider.id)}
              className="sr-only"
            />
            <span className="text-2xl mr-3">{getProviderIcon(provider.name)}</span>
            <div className="flex-1">
              <div className="font-medium capitalize">{provider.name}</div>
              <div className="text-sm text-gray-500">{provider.model}</div>
            </div>
            {provider.isLocal && (
              <span className="text-xs bg-green-100 text-green-700 px-2 py-1 rounded">
                Local
              </span>
            )}
            {provider.isDefault && (
              <span className="text-xs bg-blue-100 text-blue-700 px-2 py-1 rounded">
                Default
              </span>
            )}
          </label>
        ))}
      </div>
    </div>
  );
}
