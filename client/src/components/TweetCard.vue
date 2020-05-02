<template>
  <span v-html="this.props.html"></span>
</template>

<script>
export default {
  props: {
    html: String
  }
};
</script>
