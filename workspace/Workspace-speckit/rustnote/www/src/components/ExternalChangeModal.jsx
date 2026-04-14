import { useEffect } from 'react';
import { useTranslation } from 'react-i18next';

export default function ExternalChangeModal({ 
  isVisible, 
  fileName, 
  onReload, 
  onKeepCurrent, 
  onCompareLater,
  isLoading 
}) {
  const { t } = useTranslation();

  useEffect(() => {
    const handleKeyDown = (e) => {
      if (!isVisible) return;
      
      if (e.key === 'Escape') {
        onCompareLater();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isVisible, onCompareLater]);

  if (!isVisible) {
    return null;
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="absolute inset-0 bg-black/50" onClick={onCompareLater} />
      
      <div
        className="relative bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-md mx-4"
        style={{
          backgroundColor: 'var(--bg-primary)',
          color: 'var(--text-primary)',
        }}
      >
        <div
          className="flex items-center justify-between px-6 py-4 border-b"
          style={{ borderColor: 'var(--border-color)' }}
        >
          <h2 className="text-lg font-semibold">{t('externalChange.title')}</h2>
          <button
            onClick={onCompareLater}
            className="text-2xl leading-none opacity-60 hover:opacity-100"
            style={{ color: 'var(--text-primary)' }}
          >
            ×
          </button>
        </div>

        <div className="px-6 py-4">
          <div className="flex items-start gap-3 mb-4">
            <div
              className="flex-shrink-0 w-10 h-10 rounded-full flex items-center justify-center"
              style={{ backgroundColor: 'var(--accent-color)', opacity: 0.2 }}
            >
              <svg
                className="w-5 h-5"
                style={{ color: 'var(--accent-color)' }}
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                />
              </svg>
            </div>
            <div>
              <p className="font-medium" style={{ color: 'var(--text-primary)' }}>
                {fileName}
              </p>
              <p className="text-sm mt-1" style={{ color: 'var(--text-secondary)' }}>
                {t('externalChange.message')}
              </p>
            </div>
          </div>
        </div>

        <div
          className="flex justify-end gap-2 px-6 py-4 border-t"
          style={{ borderColor: 'var(--border-color)' }}
        >
          <button
            onClick={onCompareLater}
            disabled={isLoading}
            className="px-4 py-2 text-sm border rounded cursor-pointer transition-colors disabled:opacity-50"
            style={{
              backgroundColor: 'var(--bg-primary)',
              borderColor: 'var(--border-color)',
              color: 'var(--text-primary)',
            }}
          >
            {t('externalChange.compareLater')}
          </button>
          <button
            onClick={onKeepCurrent}
            disabled={isLoading}
            className="px-4 py-2 text-sm border rounded cursor-pointer transition-colors disabled:opacity-50"
            style={{
              backgroundColor: 'var(--bg-primary)',
              borderColor: 'var(--border-color)',
              color: 'var(--text-primary)',
            }}
          >
            {t('externalChange.keepCurrent')}
          </button>
          <button
            onClick={onReload}
            disabled={isLoading}
            className="px-4 py-2 text-sm border rounded cursor-pointer transition-colors disabled:opacity-50"
            style={{
              backgroundColor: 'var(--accent-color)',
              borderColor: 'var(--accent-color)',
              color: 'white',
            }}
          >
            {isLoading ? t('externalChange.reloading') : t('externalChange.reload')}
          </button>
        </div>
      </div>
    </div>
  );
}
