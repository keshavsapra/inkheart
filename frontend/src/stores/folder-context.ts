import { writable } from 'svelte/store';
import type { AncestorRecord } from '../types/library-item';

export type FolderContext = {
  title: string;
  ancestors: AncestorRecord[];
  onBack: () => void;
};

export const folderContext = writable<FolderContext | null>(null);

export const setFolderContext = (context: FolderContext) => {
  folderContext.set(context);
};

export const clearFolderContext = () => {
  folderContext.set(null);
};
