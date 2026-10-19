import customMedia from 'postcss-custom-media'
import inject from 'postcss-inject'

export default {
  plugins: [
    inject({
      injectTo: 'fileStart',
      cssFilePath: 'src/media-queries.css',
    }),
    customMedia(),
  ],
}
