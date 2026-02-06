import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter({
      fallback: 'index.html',
      pages: 'dist',
      assets: 'dist',
      precompress: false,
      strict: true
    })
  }
};

export default config;
