import { writable } from 'svelte/store';

export type SearchScope =
  | { type: 'global' }
  | { type: 'folder'; folderId: string; folderName: string };

export const searchScope = writable<SearchScope>({ type: 'global' });

export const setFolderScope = (folderId: string, folderName: string) => {
  searchScope.set({ type: 'folder', folderId, folderName });
};

export const setGlobalScope = () => {
  searchScope.set({ type: 'global' });
};
