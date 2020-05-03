<template>
  <section>
    <b-field horizontal label="link">
      <b-input
        v-model="link"
        placeholder="https://twitter.com/trapyojo/..."
      ></b-input>
    </b-field>
    <b-field horizontal label="comment">
      <b-input v-model="comment" type="textarea"></b-input>
    </b-field>
    <b-field horizontal label="tags">
      <b-taginput
        v-model="selectedTags"
        autocomplete
        :data="filteredTagCandidates"
        field="tag"
        allow-new
        @typing="getFilteredTags"
      ></b-taginput>
    </b-field>
    <b-button @click="postData">Register</b-button>
  </section>
</template>

<script>
export default {
  data() {
    return {
      link: '',
      comment: '',
      tagCandidates: [],
      filteredTagCandidates: [],
      selectedTags: []
    };
  },
  mounted() {
    this.$axios.get('/tags').then(resp => {
      this.tagCandidates = resp.data;
    });
  },
  methods: {
    getFilteredTags(text) {
      this.filteredTagCandidates = this.tagCandidates.filter(
        t => t.tag.toLowerCase().indexOf(text.toLowerCase()) >= 0
      );
    },
    postData() {
      const d = [];
      this.selectedTags.forEach(t => {
        if (typeof t === 'string') {
          // 新規
          this.$axios
            .post('/tags', {
              content: t
            })
            .then(resp => {
              d.push(resp.data.id);
            });
        } else {
          d.push(t.id);
        }
      });
      this.$axios
        .post('/tweets', {
          link: this.link,
          comment: this.comment,
          tags: d
        })
        .then(resp => {
          console.log(resp);
        });
    }
  }
};
</script>
