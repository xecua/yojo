<template>
  <div>
    <div v-for="(tweet, i) in tweets" :key="i">
      <tweet-card :html="tweet.html"></tweet-card>
    </div>
  </div>
</template>

<script>
import TweetCard from '@/components/TweetCard';

export default {
  data() {
    return {
      tweets: [] // TweetDetail[]
    };
  },
  mounted() {
    this.$axios.get('/tweets').then(resp => {
      this.tweets = resp.body;
    });
  },
  components: {
    'tweet-card': TweetCard
  }
};
</script>
