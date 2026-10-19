import { toast } from '@zerodevx/svelte-toast'
import UndoToast from '../ui/toasts/UndoToast.svelte'

export const pushUndoToast = (message: string, onUndo: () => void | Promise<void>) =>
  toast.push({
    component: {
      src: UndoToast,
      props: { message, onUndo },
      sendIdTo: 'toastId',
    },
    duration: 6000,
    pausable: true,
  })
