import { createTauriTest, expect } from '@srsholmes/tauri-playwright';

export const { test, describe } = createTauriTest({
  devUrl: 'http://localhost:1420',
  ipcMocks: {},
  mcpSocket: '/tmp/tauri-playwright.sock',
});

export { expect };

export async function createNewDocument(tauriPage: any) {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';
  
  await tauriPage.goto('http://localhost:1420');
  await tauriPage.waitForSelector('#editor-content', { timeout: 10000 });
  
  return { modifier };
}

export function getEditor(tauriPage: any) {
  return tauriPage.locator('#editor-content');
}
