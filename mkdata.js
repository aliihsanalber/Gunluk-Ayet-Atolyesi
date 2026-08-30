/**
 * data/qdata.json üretir (Arapça + 4 meâl + sûre adları + öne çıkanlar).
 * Kaynak: data/ar_uthmani.json + tr_diyanet.json + tr_elmalili.json + tr_yasar.json + tr_ali_bulac.json
 *   node mkdata.js
 */
const fs = require('fs');
const path = require('path');
const D = (f) => path.join(__dirname, 'data', f);
const ar = JSON.parse(fs.readFileSync(D('ar_uthmani.json'), 'utf8')).quran;

const MEAL_SOURCES = [
  { key: 'diyanet', file: 'tr_diyanet.json', name: 'Diyanet İşleri Meali', author: 'Diyanet İşleri Başkanlığı' },
  { key: 'elmalili', file: 'tr_elmalili.json', name: 'Elmalılı Hamdi Yazır Meali', author: 'Elmalılı Hamdi Yazır' },
  { key: 'feyzul', file: 'tr_feyzul.json', name: 'Feyzü\'l-Furkân Meali', author: 'Prof. Dr. Hasan Tahsin Feyizli' },
  { key: 'ali_bulac', file: 'tr_ali_bulac.json', name: 'Ali Bulaç Meali', author: 'Ali Bulaç' }
];

const meals = {};
for (const src of MEAL_SOURCES) {
  const raw = JSON.parse(fs.readFileSync(D(src.file), 'utf8'));
  const by = {};
  for (const v of raw.quran) (by[v.chapter] = by[v.chapter] || {})[v.verse] = v.text;
  meals[src.key] = { by, name: src.name, author: src.author };
}

const NAMES = ["Fâtiha","Bakara","Âl-i İmrân","Nisâ","Mâide","En'âm","A'râf","Enfâl","Tevbe","Yûnus",
"Hûd","Yûsuf","Ra'd","İbrâhim","Hicr","Nahl","İsrâ","Kehf","Meryem","Tâhâ",
"Enbiyâ","Hac","Mü'minûn","Nûr","Furkān","Şuarâ","Neml","Kasas","Ankebût","Rûm",
"Lokmân","Secde","Ahzâb","Sebe'","Fâtır","Yâsîn","Sâffât","Sâd","Zümer","Mü'min",
"Fussilet","Şûrâ","Zuhruf","Duhân","Câsiye","Ahkāf","Muhammed","Fetih","Hucurât","Kāf",
"Zâriyât","Tûr","Necm","Kamer","Rahmân","Vâkıa","Hadîd","Mücâdele","Haşr","Mümtehine",
"Saf","Cuma","Münâfikūn","Teğâbün","Talâk","Tahrîm","Mülk","Kalem","Hâkka","Meâric",
"Nûh","Cin","Müzzemmil","Müddessir","Kıyâme","İnsân","Mürselât","Nebe'","Nâziât","Abese",
"Tekvîr","İnfitâr","Mutaffifîn","İnşikāk","Bürûc","Târık","A'lâ","Ğâşiye","Fecr","Beled",
"Şems","Leyl","Duhâ","İnşirâh","Tîn","Alak","Kadir","Beyyine","Zilzâl","Âdiyât",
"Kāria","Tekâsür","Asr","Hümeze","Fîl","Kureyş","Mâûn","Kevser","Kâfirûn","Nasr",
"Tebbet","İhlâs","Felak","Nâs"];

const arBy = {};
for (const v of ar) (arBy[v.chapter] = arBy[v.chapter] || {})[v.verse] = v.text;

const counts = [], A = [];
for (let s = 1; s <= 114; s++) {
  const n = Object.keys(arBy[s]).length;
  counts.push(n);
  const a = [];
  for (let v = 1; v <= n; v++) a.push(arBy[s][v]);
  A.push(a);
}

const mealData = {};
for (const [key, src] of Object.entries(meals)) {
  const T = [];
  for (let s = 1; s <= 114; s++) {
    const n = counts[s - 1];
    const t = [];
    for (let v = 1; v <= n; v++) t.push((src.by[s][v] || '').trim());
    T.push(t);
  }
  mealData[key] = { tr: T, name: src.name, author: src.author };
}

const PICKS = [
  [2,152,152],[2,153,153],[2,186,186],[2,255,255],[2,286,286],[2,45,45],[2,216,216],[2,201,201],
  [3,159,159],[3,173,173],[3,139,139],[3,103,103],[3,134,134],[3,200,200],
  [8,46,46],[8,61,61],[13,28,28],[13,24,24],[14,7,7],[16,90,90],[16,97,97],[17,23,24],[17,80,80],
  [20,114,114],[24,35,35],[25,63,63],[28,77,77],[29,45,45],[29,69,69],
  [30,41,41],[31,18,19],[39,53,53],[39,10,10],[40,60,60],[41,34,34],[42,30,30],
  [49,10,10],[49,11,12],[49,13,13],[50,16,16],[55,13,13],[57,20,20],[64,11,11],[65,2,3],
  [76,8,9],[93,4,4],[93,9,11],[94,5,6],[103,1,3],[110,1,3],[112,1,4]
];

const OUT = {
  names: NAMES,
  counts,
  ar: A,
  meals: mealData,
  defaultMeal: 'diyanet',
  picks: PICKS
};

fs.writeFileSync(D('qdata.json'), JSON.stringify(OUT));
console.log('qdata.json', (fs.statSync(D('qdata.json')).size / 1024 / 1024).toFixed(2) + ' MB');
console.log('meals:', Object.keys(mealData).join(', '));
console.log('2:255 (Diyanet) =', mealData.diyanet.tr[1][254].slice(0, 50));
console.log('2:255 (Elmalılı) =', mealData.elmalili.tr[1][254].slice(0, 50));