<template>
  <div>
    <div class="container">
      <register-form></register-form>
    </div>
    <div>
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
      this.tweets = resp.body;
      console.log(this.tweets);
    });
  },
  components: {
    RegisterForm,
    TweetCard
  }
};
</script>
