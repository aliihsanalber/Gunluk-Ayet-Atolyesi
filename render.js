/**
 * Ayet kartı PNG üretici (Windows uyarlaması)
 * Kullanım: node render.js [--font HafsUthmanic|AdobeNaskh] [--only A,B,C]
 * Çıktı: ayet_A.png, ayet_B.png, ... (1080×değişken yükseklik @2x)
 * Font varsayılanı: HafsUthmanic (KFGQPC HAFS Uthmanic Script — 1.jpeg referansı).
 * Farklı fontla karşılaştırma: node render.js --font AdobeNaskh
 */
const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');

const args = process.argv.slice(2);
const getArg = (name) => {
  const i = args.indexOf(name);
  return i >= 0 ? args[i + 1] : null;
};
const FONT = getArg('--font') || 'HafsUthmanic';
const ONLY = getArg('--only') ? getArg('--only').split(',') : null;

const options = JSON.parse(fs.readFileSync(path.join(__dirname, 'options.json'), 'utf-8'));
const template = fs.readFileSync(path.join(__dirname, 'card_template.html'), 'utf-8');

/* Talimat kuralı: vakf/tecvid işaretleri ayıklanır (U+0615, U+06D6–U+06ED);
   harekeler (fetha/damme/kesre/şedde/sükun/tenvin, U+0670, U+0656) korunur. */
const WAQF_RE = /[ؕۖ-ۭ]/g;
const stripWaqf = (t) => t.replace(WAQF_RE, '');

/* ﴿N﴾ işareti süslü parantez rozeti olarak aynen korunur (U+06DD asla kullanılmaz).
   Her "metin ﴿N﴾" çifti ayrı blok: numara asla ayetinin son satırından kopmaz. */
function buildArabic(raw) {
  const t = stripWaqf(raw);
  const blocks = [];
  const re = /﴿(.+?)﴾/g;
  let last = 0, m;
  while ((m = re.exec(t)) !== null) {
    const text = t.slice(last, m.index).trim();
    if (text) blocks.push({ text, num: m[1] });
    last = re.lastIndex;
  }
  const tail = t.slice(last).trim();
  if (tail) blocks.push({ text: tail, num: null });
  return blocks.map((p) => {
    const badge = p.num ? `<span class="badge">﴿${p.num}﴾</span>` : '';
    return `<span class="ayah">${p.text}${badge}</span>`;
  }).join('');
}

async function renderCard(page, { source, part, arabic, meal }, outPath) {
  const html = template
    .replace(/__SOURCE__/g, source)
    .replace(/__PART__/g, part || '')
    .replace(/__PART_STYLE__/g, part ? '' : 'display:none;')
    .replace(/__ARABIC__/g, buildArabic(arabic))
    .replace(/__MEAL__/g, meal)
    .replace(/__ARFONT__/g, FONT);

  const tmp = path.join(__dirname, '_render_tmp.html');
  fs.writeFileSync(tmp, html, 'utf-8');
  await page.goto('file:///' + tmp.replace(/\\/g, '/'), { waitUntil: 'load' });
  await page.evaluate(() => document.fonts.ready.then(() => true));
  await page.waitForTimeout(300); /* fit'in oturması için */

  const metrics = await page.evaluate(() => {
    const gs = (sel) => parseFloat(getComputedStyle(document.querySelector(sel)).fontSize);
    const card = document.getElementById('card');
    return {
      arS: gs('.arabic'), mealS: gs('.meal'), srcS: gs('.title'),
      h: card.offsetHeight, overflow: card.dataset.overflow === '1',
    };
  });

  if (outPath) await page.locator('#card').screenshot({ path: outPath });
  return metrics;
}

(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 1200, height: 2200 }, deviceScaleFactor: 2 });
  const fontTag = FONT === 'HafsUthmanic' ? '' : `_${FONT.toLowerCase()}`;

  for (const key of Object.keys(options)) {
    if (ONLY && !ONLY.includes(key)) continue;
    const o = options[key];
    const parts = Array.isArray(o.parts) && o.parts.length ? o.parts : [{ arabic: o.arabic, meal: o.meal }];
    const n = parts.length;
    for (let i = 0; i < n; i++) {
      const suffix = n > 1 ? `_${i + 1}` : '';
      const partLabel = n > 1 ? `KART ${i + 1}/${n}` : '';
      const out = path.join(__dirname, `ayet_${key}${suffix}${fontTag}.png`);
      const m = await renderCard(page, { source: o.source, part: partLabel, arabic: parts[i].arabic, meal: parts[i].meal }, out);
      const warn = m.overflow ? '  ⚠ TAŞMA — ayet bölünmeli' : '';
      console.log(`ayet_${key}${suffix}${fontTag}.png  arapca=${Math.round(m.arS)}px  meal=${Math.round(m.mealS)}px  kart=${m.h}px${warn}`);
    }
  }

  await browser.close();
  try { fs.unlinkSync(path.join(__dirname, '_render_tmp.html')); } catch (e) {}
  console.log('Bitti.');
})();
