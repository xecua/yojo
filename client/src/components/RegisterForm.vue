<template>
  <section>
    <b-collapse
      class="card"
      :open="false"
      aria-id="registerForm">
      <div
        slot="trigger"
        slot-scope="props"
        class="card-header"
        role="button"
        aria-controls="registerForm">
        <p class="card-header-title">
          新規登録
        </p>
        <a class="card-header-icon">
          <b-icon
            type="is-black"
            :icon="props.open ? 'menu-down' : 'menu-up'" />
        </a>
      </div>
      <div class="card-content">
        <b-field
          horizontal
          label="link">
          <b-input
            v-model="link"
            placeholder="https://twitter.com/trapyojo/..." />
        </b-field>
        <b-field
          horizontal
          label="comment">
          <b-input
            v-model="comment"
            type="textarea" />
        </b-field>
        <b-field
          horizontal
          label="tags">
          <b-taginput
            v-model="selectedTags"
            autocomplete
            :data="filteredTagCandidates"
            field="tag"
            allow-new
            @typing="getFilteredTags" />
        </b-field>
        <b-button @click="postData">
          Register
        </b-button>
      </div>
    </b-collapse>
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
    this.updateTags();
  },
  methods: {
    getFilteredTags(text) {
      this.filteredTagCandidates = this.tagCandidates.filter(
        t => t.tag.toLowerCase().indexOf(text.toLowerCase()) >= 0
      );
    },
    postData() {
      Promise.all(
        this.selectedTags.map(t => {
          if (typeof t === 'string') {
            // 新規
            return this.$axios
              .post('/tags', {
                content: t
              })
              .then(resp => resp.data.id);
          } else {
            new Promise(resolve => resolve(t.id));
          }
        })
      ).then(d => {
        this.$axios
          .post('/tweets', {
            link: this.link,
            comment: this.comment,
            tags: d
          })
          .then(resp => {
            console.log(resp);
          });
      });
    }
  }
};
</script>
