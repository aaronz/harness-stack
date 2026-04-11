import { createContext, useContext, useState, useCallback } from 'react';

const SearchContext = createContext();

export function SearchProvider({ children }) {
  const [isSearchVisible, setIsSearchVisible] = useState(false);

  const showSearch = useCallback(() => {
    setIsSearchVisible(true);
  }, []);

  const hideSearch = useCallback(() => {
    setIsSearchVisible(false);
  }, []);

  const toggleSearch = useCallback(() => {
    setIsSearchVisible(prev => !prev);
  }, []);

  return (
    <SearchContext.Provider value={{
      isSearchVisible,
      showSearch,
      hideSearch,
      toggleSearch,
    }}>
      {children}
    </SearchContext.Provider>
  );
}

export function useSearch() {
  const context = useContext(SearchContext);
  if (!context) {
    throw new Error('useSearch must be used within SearchProvider');
  }
  return context;
}