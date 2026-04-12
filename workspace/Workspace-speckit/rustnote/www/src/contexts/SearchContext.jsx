import { createContext, useContext, useState, useCallback, useRef } from 'react';

const SearchContext = createContext();

export function SearchProvider({ children }) {
  const [isSearchVisible, setIsSearchVisible] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [currentMatch, setCurrentMatch] = useState(0);
  const [matchCount, setMatchCount] = useState(0);
  const findNextRef = useRef(null);
  const findPrevRef = useRef(null);

  const showSearch = useCallback(() => {
    setIsSearchVisible(true);
  }, []);

  const hideSearch = useCallback(() => {
    setIsSearchVisible(false);
  }, []);

  const toggleSearch = useCallback(() => {
    setIsSearchVisible(prev => !prev);
  }, []);

  const registerSearchFunctions = useCallback((findNext, findPrev) => {
    findNextRef.current = findNext;
    findPrevRef.current = findPrev;
  }, []);

  const findNext = useCallback(() => {
    if (findNextRef.current) {
      findNextRef.current();
    }
  }, []);

  const findPrev = useCallback(() => {
    if (findPrevRef.current) {
      findPrevRef.current();
    }
  }, []);

  return (
    <SearchContext.Provider value={{
      isSearchVisible,
      showSearch,
      hideSearch,
      toggleSearch,
      registerSearchFunctions,
      findNext,
      findPrev,
      searchQuery,
      setSearchQuery,
      currentMatch,
      setCurrentMatch,
      matchCount,
      setMatchCount,
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