/**
 * assets/cerceve2.png (AI üretimi, krem zeminli süslü köşe çerçevesi)
 *   -> assets/cerceve2-clear.png (krem zemin şeffaf, sadece altın köşeler)
 *
 * Böylece süslü çerçeve herhangi bir zemin rengiyle birlikte kullanılabilir.
 * Başka bir çerçeve denemek için: assets/cerceve2.png yerine yenisini koy,
 * gerekirse aşağıdaki eşikleri (CREAM, ac/al katsayıları) ayarla, tekrar çalıştır.
 *
 *   npm i pngjs        (bir kez)
 *   node make-frame.js
 */
const fs = require('fs');
const path = require('path');
const { PNG } = require('pngjs');

const SRC = path.join(__dirname, 'assets', 'cerceve2.png');
const OUT = path.join(__dirname, 'assets', 'cerceve2-clear.png');

const s = PNG.sync.read(fs.readFileSync(SRC));
const out = new PNG({ width: s.width, height: s.height });
const CREAM = [247, 240, 228];
const cl = (v, a, b) => Math.max(a, Math.min(b, v));

for (let i = 0; i < s.data.length; i += 4) {
  const r = s.data[i], g = s.data[i + 1], b = s.data[i + 2];
  const avg = (r + g + b) / 3, rb = r - b;
  // altın: sıcak (rb yüksek) VEYA belirgin koyu
  let ac = cl((rb - 30) * 10, 0, 255);
  let al = rb > 12 ? cl((208 - avg) * 6, 0, 255) : 0;
  let a = Math.max(ac, al) / 255;
  if (a < 0.06) { out.data[i + 3] = 0; continue; }
  // krem katkısını çöz (temiz altın)
  out.data[i]     = cl(Math.round((r - CREAM[0] * (1 - a)) / a), 0, 255);
  out.data[i + 1] = cl(Math.round((g - CREAM[1] * (1 - a)) / a), 0, 255);
  out.data[i + 2] = cl(Math.round((b - CREAM[2] * (1 - a)) / a), 0, 255);
  out.data[i + 3] = Math.round(a * 255);
}
fs.writeFileSync(OUT, PNG.sync.write(out));
console.log('yazıldı', (fs.statSync(OUT).size / 1024 | 0) + ' KB ->', path.relative(process.cwd(), OUT));
