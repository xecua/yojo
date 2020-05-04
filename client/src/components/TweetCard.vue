<template>
  <div class="tile is-ancestor">
    <div
      class="tile is-child has-text-right"
      v-html="html" />
    <div class="tile is-parent is-vertical has-text-left">
      <div class="tile is-child">
        <h5 class="is-size-5">
          Comment
        </h5>
        <p>{{ comment }}</p>
      </div>
      <div class="tile is-child">
        <h5 class="is-size-5">
          Tag
        </h5>
        <b-taglist
          v-for="(tag, i) in tags"
          :key="i">
          <b-tag
            class="search-tag"
            @click.native="$emit('tag-clicked', tag)">
            {{ tag.tag }}
          </b-tag>
        </b-taglist>
      </div>
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
  }
};
</script>

<style lang="stylus" scoped>
.search-tag
  cursor: pointer
</style>
