import { toast } from '@zerodevx/svelte-toast';
import { get } from 'svelte/store';
import TextIndexProgressToast from '../ui/toasts/TextIndexProgressToast.svelte';
import { IndexerWatcher, type TextIndexProgress } from './indexer-watcher';

let activeToastId: number | null = null;

const showToast = (progress: TextIndexProgress) => {
  if (activeToastId === null) {
    activeToastId = toast.push({
      component: {
        src: TextIndexProgressToast,
        props: {
          total: progress.total,
          processed: progress.processed,
        },
        sendIdTo: 'toastId',
      },
      dismissable: false,
      duration: 0,
      initial: 0,
    });
  } else {
    toast.set(activeToastId, {
      component: {
        src: TextIndexProgressToast,
        props: {
          total: progress.total,
          processed: progress.processed,
        },
        sendIdTo: 'toastId',
      },
    });
  }
};

const dismissToast = () => {
  if (activeToastId !== null) {
    toast.pop(activeToastId);
    activeToastId = null;
  }
};

export const initTextIndexToast = () => {
  IndexerWatcher.textIndexProgress.subscribe((progress: TextIndexProgress) => {
    if (progress.active) {
      showToast(progress);
    } else if (activeToastId !== null) {
      dismissToast();
    }
  });

  IndexerWatcher.checkOnce();
};
