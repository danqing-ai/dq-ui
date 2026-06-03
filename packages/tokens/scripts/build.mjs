import { mkdir, copyFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execSync } from 'node:child_process';

const rootDir = dirname(fileURLToPath(import.meta.url));
const packageDir = resolve(rootDir, '..');
const srcDir = resolve(packageDir, 'src');
const distDir = resolve(packageDir, 'dist');

await mkdir(distDir, { recursive: true });

for (const name of [
  'dq-mac.css',
  'dq-glass.css',
  'dq-linear-dark.css',
  'dq-china-red-dark.css',
  'dq-tauri-macos.css',
]) {
  await copyFile(resolve(srcDir, name), resolve(distDir, name));
}

execSync('tsc -p tsconfig.json', { cwd: packageDir, stdio: 'inherit' });
