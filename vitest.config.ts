import { defineConfig } from 'vitest/config';
import vue from '@vitejs/plugin-vue';
import { resolve } from 'path';

export default defineConfig({
  plugins: [vue()],
  test: {
    environment: 'happy-dom',
    globals: true,
    deps: {
      inline: ['@vue']
    }
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
      '@assets': resolve(__dirname, './src/assets'),
      '@components': resolve(__dirname, './src/components'),
      '@layouts': resolve(__dirname, './src/components/layouts'),
      '@mods': resolve(__dirname, './src/components/mods'),
      '@modals': resolve(__dirname, './src/components/modals'),
      '@common': resolve(__dirname, './src/components/common'),
      '@stores': resolve(__dirname, './src/stores'),
      '@services': resolve(__dirname, './src/services'),
      '@main-types': resolve(__dirname, './src/types'),
      '@utils': resolve(__dirname, './src/utils'),
    },
  }
});