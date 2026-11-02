# Re-extracting from OverPy

Only needed to refresh the data from a newer OverPy. Requires Node 22 and pnpm.

```bash
git clone https://github.com/spoo-bar/overpy && cd overpy && pnpm install
sed "s#OVERPY_SRC#$PWD/src#g" ../ow-workshop/data/extract/entry.ts > /tmp/entry.ts
npx esbuild /tmp/entry.ts --bundle --platform=node --external:vscode --outfile=/tmp/extract.js
node /tmp/extract.js /tmp/raw.json src/compiler/functions/createCasedProgressBarIwt.ts
node ../ow-workshop/data/extract/transform.mjs /tmp/raw.json ../ow-workshop/data
```

`transform.mjs` exits non-zero on unresolved defaults or missing English names.
