<template>
  <span v-html="this.html"></span>
</template>

<script>
export default {
  props: {
    html: String
  }
};
</script>
