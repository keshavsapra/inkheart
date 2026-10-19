<style>
  .modal {
    position: fixed;

    background: none;
    padding: 0;
    border: none;
    transform: translateY(-5%);

    /* override defaults */
    max-width: calc(100vw - 2rem);
    max-width: calc(100dvw - 2rem);
  }

  .modal.behind {
    visibility: hidden;
  }

  .modal.behind::backdrop {
    background: transparent;
  }

  @media (max-width: 768px) {
    .modal {
      transform: none;
    }
  }
</style>

<script lang="ts">
  import { onDestroy } from 'svelte'
  import Card from '../card/Card.svelte'
  import { modalStack } from '../../../stores/modal-stack'

  export let open = false

  let dialog: HTMLDialogElement
  const id = Symbol()

  $: behind = open && $modalStack.includes(id) && $modalStack[$modalStack.length - 1] !== id

  $: if (open) {
    modalStack.add(id)
  } else {
    modalStack.remove(id)
  }

  $: open, dialog, syncDialog()

  const syncDialog = () => {
    if (!dialog) {
      return
    }

    try {
      if (open && !dialog.open) {
        dialog.showModal()
      } else if (!open && dialog.open) {
        dialog.close()
      }
    } catch (e) {}
  }

  onDestroy(() => {
    modalStack.remove(id)
  })
</script>

<dialog bind:this="{dialog}" class="modal" class:behind="{behind}" on:close="{() => (open = false)}">
  <Card padding="{false}">
    <slot />
  </Card>
</dialog>
