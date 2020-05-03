<template>
  <div id="app">
    <div class="container">
      <register-form></register-form>
    </div>
    <div class="container">
      <div v-for="(tweet, i) in tweets" :key="i">
        <tweet-card :html="tweet.html"></tweet-card>
      </div>
    </div>
  </div>
</template>

<script>
import RegisterForm from '@/components/RegisterForm';
import TweetCard from '@/components/TweetCard';

export default {
  data() {
    return {
      tweets: [] // TweetDetail[]
    };
  },
  created() {
    this.$axios.get('/tweets').then(resp => {
      this.tweets = resp.data;
    });
  },
  components: {
    RegisterForm,
    TweetCard
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
