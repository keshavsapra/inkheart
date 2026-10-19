import { writable } from 'svelte/store';
import { createGetRequest } from '../lib/create-request';
import { showIndexerFinishedToast } from './indexer-service';
import { apiUrl } from '../utils/api';

interface IndexerStatusResponse {
  running: boolean;
  text_indexing: boolean;
  text_index_total: number;
  text_index_processed: number;
}

export interface TextIndexProgress {
  active: boolean;
  total: number;
  processed: number;
}

const createIndexerWatcher = () => {
  const { subscribe, set } = writable(false);
  const textIndexProgress = writable<TextIndexProgress>({
    active: false,
    total: 0,
    processed: 0,
  });

  let timeoutId: ReturnType<typeof setTimeout> | null = null;
  let isPolling = false;

  const checkIndexer = async (): Promise<IndexerStatusResponse> => {
    const res = await fetch(...createGetRequest(apiUrl('api/indexer/status')));
    return await res.json();
  };

  const updateTextIndexProgress = (status: IndexerStatusResponse) => {
    textIndexProgress.set({
      active: status.text_indexing,
      total: status.text_index_total,
      processed: status.text_index_processed,
    });
  };

  const poll = async (): Promise<void> => {
    if (isPolling) {
      return;
    }

    isPolling = true;

    try {
      const status = await checkIndexer();
      set(status.running);
      updateTextIndexProgress(status);

      if (!status.running && !status.text_indexing) {
        stop();
        showIndexerFinishedToast();
      } else {
        timeoutId = setTimeout(poll, 2000);
      }
    } catch {
      stop();
      set(false);
    } finally {
      isPolling = false;
    }
  };

  const start = async (): Promise<void> => {
    if (timeoutId) {
      return;
    }

    await poll();
  };

  const stop = (): void => {
    if (!timeoutId) {
      return;
    }

    clearTimeout(timeoutId);
    timeoutId = null;
  };

  const checkOnce = async (): Promise<void> => {
    try {
      const status = await checkIndexer();
      updateTextIndexProgress(status);

      if (status.text_indexing && !timeoutId) {
        await start();
      }
    } catch {
      // ignore
    }
  };

  return {
    subscribe,
    start,
    stop,
    checkOnce,
    textIndexProgress,
  };
};

export const IndexerWatcher = createIndexerWatcher();
