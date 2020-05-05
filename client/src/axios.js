import axios from 'axios';

export default axios.create({
  baseURL: process.env.NODE_ENV === 'production'
    ? 'https://tools.koffein.dev/dictionary/trapyojo/api'
    : 'http://localhost:3000'
});

