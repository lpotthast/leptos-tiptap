const { defineConfig } = require('@playwright/test');

module.exports = defineConfig({
  testDir: './tests',
  fullyParallel: false,
  use: {
    baseURL: 'http://127.0.0.1:3000',
    headless: true,
  },
  webServer: {
    command: 'cargo leptos watch',
    cwd: '..',
    url: 'http://127.0.0.1:3000',
    reuseExistingServer: false,
    timeout: 120000,
  },
});
