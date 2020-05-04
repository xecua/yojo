<template>
  <div class="columns is-vcentered">
    <div class="column" v-html="this.html"></div>
    <div class="column has-text-left">
      <div>{{ comment }}</div>
      <b-taglist v-for="(tagName, i) in tagNames" :key="i">
        <b-tag>{{ tagName }}</b-tag>
      </b-taglist>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    id: String,
    html: String,
    comment: String,
    tags: Array
  },
  computed: {
    tagNames() {
      const res = this.tags.map(t => t.tag);
      console.log(res);
      return res;
    }
  }
};
</script>
