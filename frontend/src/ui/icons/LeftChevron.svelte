<script lang="ts">
  export let primaryColor = 'black'
  export let width = '2rem'
  export let height = '2rem'
</script>

<div style="width: {width}; height: {height};">
  <svg width="100%" height="100%" viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg">
    <polyline
      points="328 112 184 256 328 400"
      style="fill:none;stroke:{primaryColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:48px"
    ></polyline>
  </svg>
</div>
