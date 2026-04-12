import { useToast } from '../contexts/ToastContext';

export default function Toast() {
  const { toasts, removeToast } = useToast();

  if (toasts.length === 0) return null;

  return (
    <div className="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
      {toasts.map((toast) => (
        <div
          key={toast.id}
          className={`px-4 py-3 rounded-lg shadow-lg flex items-center gap-3 min-w-[280px] max-w-md ${
            toast.type === 'error' ? 'bg-red-500 text-white' :
            toast.type === 'success' ? 'bg-green-500 text-white' :
            'bg-gray-800 text-white'
          }`}
        >
          <span className="flex-1 text-sm">{toast.message}</span>
          <button
            onClick={() => removeToast(toast.id)}
            className="text-lg leading-none opacity-70 hover:opacity-100"
          >
            ×
          </button>
        </div>
      ))}
    </div>
  );
}