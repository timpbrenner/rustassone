<template>
  <div v-bind:class="classes" @click="rotate" />
</template>

<script>
export default {
  props: ['currentTile', 'rotateTile'],
  methods: {
    rotate() {
      this.rotateTile();
    }
  },
  computed: {
    classes() {
      return ['current-tile'].concat(TileHelper.sideClasses(this.currentTile)).join(' ');
    }
  }
}
</script>
