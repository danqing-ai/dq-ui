import { mkdir, copyFile, readdir } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execSync } from 'node:child_process';

const rootDir = dirname(fileURLToPath(import.meta.url));
const packageDir = resolve(rootDir, '..');
const srcDir = resolve(packageDir, 'src');
const distDir = resolve(packageDir, 'dist');

await mkdir(distDir, { recursive: true });

// Copy all CSS files from src to dist
const srcFiles = await readdir(srcDir);
for (const name of srcFiles) {
  if (name.endsWith('.css')) {
    await copyFile(resolve(srcDir, name), resolve(distDir, name));
  }
}

execSync('tsc -p tsconfig.json', { cwd: packageDir, stdio: 'inherit' });
