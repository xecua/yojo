<template>
  <div id="app">
    <section class="section">
      <div class="container">
        <register-form />
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
    this.$axios.get('/tweets').then(resp => {
      this.tweets = resp.data;
      console.log(resp.data);
    });
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
