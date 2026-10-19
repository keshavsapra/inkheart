import { writable, derived, get, type Readable } from 'svelte/store';
import type { SearchResultItem, FullTextResult, LibraryItem } from '../types/library-item';
import { createGetRequest } from './create-request';
import { apiUrl } from '../utils/api';
import { searchModeStore, type SearchMode } from '../stores/options';
import { searchScope } from '../stores/search-scope';

function debounce<T extends (...args: any[]) => void>(fn: T, ms: number): T {
  let timer: ReturnType<typeof setTimeout>;
  return ((...args: any[]) => {
    clearTimeout(timer);
    timer = setTimeout(() => fn(...args), ms);
  }) as unknown as T;
}

export function createSearchManager() {
  const searchActive = writable(false);
  const searchResults = writable<SearchResultItem[]>([]);
  const searchFilter = writable('');
  const loadingSearch = writable(false);

  let searchGeneration = 0;

  const filterByType = (items: SearchResultItem[], mode: SearchMode) =>
    items.filter((it) => (mode === 'file' ? !it.is_dir : mode === 'dir' ? it.is_dir : true));

  const filteredResults: Readable<SearchResultItem[]> = derived(
    [searchResults, searchModeStore],
    ([$searchResults, $searchMode]) => {
      return filterByType($searchResults, $searchMode);
    }
  );

  const executeSearch = async (query: string) => {
    if (!query.trim()) {
      searchActive.set(false);
      searchResults.set([]);
      return;
    }

    const generation = ++searchGeneration;

    searchActive.set(true);
    loadingSearch.set(true);

    try {
      const encoded = encodeURIComponent(query);
      const scope = get(searchScope);
      const scopeParam =
        scope.type === 'folder' ? `&scope=${encodeURIComponent(scope.folderId)}` : '';

      const [filenameResponse, fulltextResponse] = await Promise.allSettled([
        fetch(...createGetRequest(apiUrl(`api/search?q=${encoded}${scopeParam}`))),
        fetch(...createGetRequest(apiUrl(`api/search/fulltext?q=${encoded}${scopeParam}`))),
      ]);

      if (generation !== searchGeneration) {
        return;
      }

      const filenameResults: SearchResultItem[] = [];
      const fulltextResults: SearchResultItem[] = [];

      if (filenameResponse.status === 'fulfilled' && filenameResponse.value.ok) {
        const items: LibraryItem[] = await filenameResponse.value.json();
        for (const item of items) {
          filenameResults.push({ ...item, search_source: 'filename' });
        }
      }

      if (fulltextResponse.status === 'fulfilled' && fulltextResponse.value.ok) {
        const items: FullTextResult[] = await fulltextResponse.value.json();
        for (const item of items) {
          fulltextResults.push({
            id: item.doc_id,
            filename: '',
            title: item.title,
            cover_name: item.cover_name,
            is_dir: item.is_dir,
            dir_previews: [],
            children: [],
            last_modified: 0,
            created: 0,
            search_source: 'content',
            page_num: item.page_num,
            match_count: item.match_count,
            pages: item.pages,
          });
        }
      }

      // Merge: filename matches take priority, deduplicate by id
      const seen = new Set<string>();
      const merged: SearchResultItem[] = [];

      for (const item of filenameResults) {
        seen.add(item.id);
        merged.push(item);
      }

      for (const item of fulltextResults) {
        if (!seen.has(item.id)) {
          seen.add(item.id);
          merged.push(item);
        }
      }

      searchResults.set(merged);
    } catch (error) {
      console.error('Search failed:', error);
      searchResults.set([]);
    }

    loadingSearch.set(false);
  };

  const debouncedSearch = debounce(executeSearch, 300);

  const doSearch = (query: string) => {
    searchFilter.set(query);

    if (!query.trim()) {
      searchActive.set(false);
      searchResults.set([]);
      return;
    }

    searchActive.set(true);
    loadingSearch.set(true);
    debouncedSearch(query);
  };

  const onSearchChange = (e: Event) => {
    const target = e.target as HTMLInputElement;
    doSearch(target.value);
  };

  const onClear = () => {
    searchActive.set(false);
    searchResults.set([]);
    searchFilter.set('');
  };

  // Re-run the search whenever the scope changes, so switching scope from
  // the popover refreshes results without the user retyping.
  let lastScope = get(searchScope);
  searchScope.subscribe((scope) => {
    if (scope === lastScope) {
      return;
    }

    lastScope = scope;

    const query = get(searchFilter);

    if (query.trim() && get(searchActive)) {
      loadingSearch.set(true);
      debouncedSearch(query);
    }
  });

  return {
    searchActive,
    searchResults,
    searchFilter,
    loadingSearch,
    filteredResults,
    onSearchChange,
    onClear,
  };
}
