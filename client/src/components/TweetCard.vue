<template>
  <div class="columns is-vcentered">
    <div
      class="column"
      v-html="html" />
    <div class="column has-text-left">
      <div>{{ comment }}</div>
      <b-taglist
        v-for="(tagName, i) in tagNames"
        :key="i">
        <b-tag>{{ tagName }}</b-tag>
      </b-taglist>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    id: {
      type: String,
      default: ''
    },
    html: {
      type: String,
      default: ''
    }, comment:{
      type: String,
      default: ''
    },
    tags: {
      type: Array,
      default: () => []
    }
  },
  computed: {
    tagNames() {
      return this.tags.map(t => t.tag);
    }
  }
};
</script>
