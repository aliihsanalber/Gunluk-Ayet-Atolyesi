/**
 * Günün Ayeti Atölyesi — build
 * gunun-ayeti-atolyesi.src.html + gömülü varlıklar -> dist/gunun-ayeti-atolyesi.html
 *
 *   node build.js
 *
 * src dosyasındaki yer tutucular:
 *   @@FONT_B64@@   -> assets/HafsUthmanicScript-Regular.otf (base64)
 *   @@FRAME_B64@@  -> assets/cerceve2-clear.png (base64, şeffaf köşeli süslü çerçeve)
 *   @@QDATA_JSON@@ -> data/qdata.json  (tüm Kur'an: Arapça + meâl + sûre adları + öne çıkanlar)
 *   @@NEWS_JSON@@  -> data/news.json   (sol Gündem paneli; routine her sabah canlı sürümü günceller)
 */
const fs = require('fs');
const path = require('path');
const R = (...p) => path.join(__dirname, ...p);

let html = fs.readFileSync(R('gunun-ayeti-atolyesi.src.html'), 'utf8');

const fontB64  = fs.readFileSync(R('assets', 'HafsUthmanicScript-Regular.otf')).toString('base64');
const frameB64 = fs.readFileSync(R('assets', 'cerceve2-clear.png')).toString('base64');
const qdata    = fs.readFileSync(R('data', 'qdata.json'), 'utf8').trim();
const news     = fs.readFileSync(R('data', 'news.json'), 'utf8').trim();

for (const [name, v] of [['qdata', qdata], ['news', news]]) {
  JSON.parse(v); // geçerlilik
  if (v.includes('</script')) { console.error(name + ' içinde </script var'); process.exit(1); }
}

html = html.replace('@@FONT_B64@@', fontB64);
html = html.replace('@@FRAME_B64@@', frameB64);
html = html.replace('@@QDATA_JSON@@', () => qdata);
html = html.replace('@@NEWS_JSON@@', () => news);

for (const tok of ['@@FONT_B64@@', '@@FRAME_B64@@', '@@QDATA_JSON@@', '@@NEWS_JSON@@']) {
  if (html.includes(tok)) { console.error('Yer tutucu kaldı:', tok); process.exit(1); }
}

const out = R('dist', 'gunun-ayeti-atolyesi.html');
fs.writeFileSync(out, html, 'utf8');
console.log('yazıldı', (html.length / 1048576).toFixed(2) + ' MB ->', path.relative(process.cwd(), out));

/* PWA dosyalarını dist'e kopyala */
for (const f of ['manifest.json', 'sw.js']) {
  fs.copyFileSync(R(f), R('dist', f));
  console.log('  +', f);
}
for (const f of ['icon-192.png', 'icon-512.png']) {
  fs.copyFileSync(R('assets', f), R('dist', f));
  console.log('  +', f);
}
