import { mkdir, readFile, writeFile, copyFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

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

const indexTs = await readFile(resolve(srcDir, 'index.ts'), 'utf8');
const matchedVersion = indexTs.match(/DQ_TOKENS_VERSION\s*=\s*['"]([^'"]+)['"]/);
const version = matchedVersion?.[1] ?? '0.1.0';

await writeFile(
  resolve(distDir, 'index.js'),
  [
    "/** Import in app entry: `import '@danqing/dq-tokens/dq-mac.css'` */",
    `export const DQ_TOKENS_VERSION = '${version}';`,
    '',
  ].join('\n'),
  'utf8',
);

await writeFile(
  resolve(distDir, 'index.d.ts'),
  [
    "/** Import in app entry: `import '@danqing/dq-tokens/dq-mac.css'` */",
    `export declare const DQ_TOKENS_VERSION: '${version}';`,
    '',
  ].join('\n'),
  'utf8',
);
