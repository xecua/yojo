<template>
  <div id="app">
    <section class="section">
      <div class="columns is-centered">
        <b-field class="column is-6">
          <b-input
            v-model="searchQuery"
            rounded
            icon="magnify" />
        </b-field>
      </div>
    </section>

    <template v-if="searchQuery.length > 0">
      <section class="section">
        <div class="container">
          <template v-for="(tweet, i) in filteredTweets">
            <tweet-card
              :key="i"
              v-bind="tweet"
              @tag-clicked="onTagClicked" />
          </template>
        </div>
      </section>
    </template>

    <template v-else>
      <section class="section">
        <div class="container">
          <register-form @tweet-posted="updateTweet" />
        </div>
      </section>
      <section class="section">
        <div class="container">
          <template v-for="(tweet, i) in tweets">
            <tweet-card
              :key="i"
              v-bind="tweet"
              @tag-clicked="onTagClicked" />
          </template>
        </div>
      </section>
    </template>
  </div>
</template>

<script>
import RegisterForm from '@/components/RegisterForm';
import TweetCard from '@/components/TweetCard';

export default {
  components: {
    RegisterForm,
    TweetCard
  },
  data() {
    return {
      searchQuery: '',
      tweets: [] // TweetDetail[]
    };
  },
  computed: {
    filteredTweets() {
      if (/^tag-id:.*/i.test(this.searchQuery)) {
        // タグID検索
        const id = this.searchQuery.slice(7);
        return this.tweets.filter(tweet => tweet.tags.filter(tag => tag.id === id).length > 0);
      } else if (/^tag:.*/i.test(this.searchQuery)) {
        const query = this.searchQuery.slice(4);
        return this.tweets.filter(tweet => tweet.tags.filter(tag => tag.tag.includes(query)).length > 0);
      } else {
        // テキスト検索
        const parser = new DOMParser();
        return this.tweets.filter(tweet => 
          parser.parseFromString(tweet.html, 'text/html')
            .getElementsByTagName('p')[0]
            .innerText
            .includes(this.searchQuery)
        );
      }
    }
  },
  created() {
    this.updateTweet();
  },
  updated() {
    // eslint-disable-next-line no-undef
    if (typeof twttr !== 'undefined') {
      // eslint-disable-next-line no-undef
      twttr.widgets.load(); // from widgets.js
    }
  },
  methods: {
    onTagClicked(tag) {
      this.searchQuery = `tag-id:${tag.id}`;
    },
    updateTweet() {
      this.$axios.get('/tweets').then(resp => {
        this.tweets = resp.data;
      });
    }
  }
};
</script>

<style lang="stylus">
#app
  font-family Avenir, Helvetica, Arial, sans-serif
  -webkit-font-smoothing antialiased
  -moz-osx-font-smoothing grayscale
  text-align center
  color #2c3e50
  margin-top 60px
</style>
