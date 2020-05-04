<template>
  <div id="app">
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
            v-bind="tweet" />
        </template>
      </div>
    </section>
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
      tweets: [] // TweetDetail[]
    };
  },
  created() {
    this.updateTweet();
  },
  methods: {
    updateTweet() {
      this.$axios.get('/tweets').then(resp => {
        this.tweets = resp.data;
        // eslint-disable-next-line no-undef
        twttr.widgets.load(); // from widgets.js
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
