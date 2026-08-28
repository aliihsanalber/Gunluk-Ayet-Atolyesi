# "Günün Ayeti" — Otonom Hazırlık Talimatı (Skill Dosyası)

Ali için her akşam otonom çalışan bir "günün ayeti" hazırlama görevi. Ali her sabah namazından önce 05:00'te bir WhatsApp grubunda ayet görseli paylaşıyor. Görev: yarın sabah paylaşılabilecek 3 alternatif (A/B/C) hazırlamak. Onay beklenmez, tamamen otonom çalışılır ve bitirilir.

## KESİN YASAKLAR

- `create_trigger` / `update_trigger` / `delete_trigger` gibi zamanlanmış görev araçları KULLANILMAZ.
- WebFetch veya bir URL'yi doğrudan açan HERHANGİ bir araç KULLANILMAZ (kuran.diyanet.gov.tr dahil). Kimse başında olmadığı için onay ekranında sonsuza kadar takılınır ve görev hiç tamamlanmaz.
- Sadece WebSearch kullanılabilir (onay gerektirmez).
- Arapça ayet metinleri ve mealleri için kendi bilgiye güvenilir; web'den doğrulamaya çalışılmaz. Arapça metin Diyanet/Türk mushafı imlasıyla yazılır (اِ / اٰ / ٖ tarzı, harekeli).

## ADIM 1: TARİH VE GÜNDEM

Yarının tarihi belirlenir. WebSearch ile:

(a) Yarın belirgin bir dini gün/gece var mı? (Ramazan, Ramazan/Kurban Bayramı, cuma, kandiller: Mevlid-Regaib-Miraç-Berat-Kadir, Aşure, Muharrem)

(b) Son 1-2 günde İslam alemini ilgilendiren gerçekten öne çıkan taze bir gündem var mı? Gazze/Filistin gibi kronik konular ancak belirgin ve YENİ bir kırılma varsa sayılır; sıradan haber akışı sayılmaz.

## ADIM 2: SEÇİM (TEKRARI ÖNLEYEN ROTASYON)

Her çalıştırma temiz bir ortamda başlar, geçmiş dosyada tutulamaz. Bu yüzden TARİH TABANLI ROTASYON kullanılır:

```
from datetime import date
d = (YARININ_TARİHİ - date(2026,1,1)).days
A = havuz[d % 99]; B = havuz[(d+33) % 99]; C = havuz[(d+66) % 99]
```
(havuz 1'den numaralı: index 0 → 1. madde)

**İSTİSNA:** Adım 1'de belirgin bir dini gün/gece VEYA gerçekten öne çıkan bir gündem varsa, A seçeneği rotasyon yerine "ÖZEL GÜN AYETLERİ"nden uygun olanla değiştirilir. B ve C rotasyondan gelmeye devam eder.

### 99'LUK HAVUZ

1. 2-Bakara:153 sabır ve namaz | 2. 94-İnşirâh:5-6 güçlükle beraber kolaylık | 3. 2-Bakara:286 güç yetmeyecek yük | 4. 39-Zümer:10 sabredenlere hesapsız ecir | 5. 3-Âl-i İmrân:200 sabır ve sebat | 6. 8-Enfâl:46 çekişmeyin | 7. 2-Bakara:155-157 imtihan ve müjde | 8. 13-Ra'd:24 selam olsun | 9. 70-Meâric:5 güzel sabır | 10. 65-Talâk:2-3 takva ve tevekkül

11. 3-Âl-i İmrân:159 tevekkül | 12. 9-Tevbe:51 Allah'ın yazdığı | 13. 33-Ahzâb:3 vekil olarak Allah | 14. 14-İbrâhim:7 şükrederseniz artırırım | 15. 2-Bakara:152 beni anın | 16. 31-Lokmân:12 şükreden kendisi için | 17. 16-Nahl:18 nimetleri sayamazsınız | 18. 55-Rahmân:13 hangi nimeti | 19. 13-Ra'd:28 kalpler zikirle huzur bulur | 20. 2-Bakara:186 ben yakınım

21. 40-Gâfir:60 bana dua edin | 22. 7-A'râf:55 yalvara yakara dua | 23. 39-Zümer:53 ümit kesmeyin | 24. 2-Bakara:222 tövbe edenleri sever | 25. 66-Tahrîm:8 nasuh tövbesi | 26. 42-Şûrâ:25 tövbeyi kabul eden | 27. 24-Nûr:22 affetsinler | 28. 12-Yûsuf:87 rahmetten ümit kesmeyin | 29. 16-Nahl:90 adalet ve ihsan | 30. 4-Nisâ:58 emanet ve adalet

31. 5-Mâide:8 kin adaletten saptırmasın | 32. 49-Hucurât:11 alay etmeyin | 33. 49-Hucurât:12 zan ve gıybet | 34. 49-Hucurât:10 müminler kardeştir | 35. 49-Hucurât:13 en üstününüz takva sahibi | 36. 17-İsrâ:23-24 anne babaya öf deme | 37. 31-Lokmân:18-19 kibirle yürüme | 38. 25-Furkān:63 tevazu ile yürür | 39. 41-Fussilet:34 kötülüğü en güzelle sav | 40. 3-Âl-i İmrân:134 öfkesini yutanlar

41. 42-Şûrâ:40 affedenin ecri | 42. 23-Mü'minûn:96 kötülüğü güzellikle sav | 43. 55-Rahmân:60 iyiliğin karşılığı iyilik | 44. 2-Bakara:83 güzel söz söyleyin | 45. 17-İsrâ:53 en güzel sözü | 46. 33-Ahzâb:70 doğru söz | 47. 2-Bakara:261 yedi başak | 48. 2-Bakara:274 gece gündüz infak | 49. 3-Âl-i İmrân:92 sevdiğinizden verin | 50. 57-Hadîd:7 iman edin ve infak edin

51. 63-Münâfikūn:10 ölüm gelmeden infak | 52. 2-Bakara:267 temizinden verin | 53. 76-İnsân:8-9 yoksula yetime esire | 54. 107-Mâûn:1-3 yetimi itip kakan | 55. 93-Duhâ:9-11 yetimi ezme | 56. 112-İhlâs:1-4 tevhid | 57. 2-Bakara:255 Âyetü'l-Kürsî | 58. 59-Haşr:22-24 Esmâü'l-Hüsnâ | 59. 57-Hadîd:4 nerede olsanız O sizinle | 60. 50-Kāf:16 şah damarından yakın

61. 6-En'âm:59 gaybın anahtarları | 62. 24-Nûr:35 Nûr âyeti | 63. 20-Tâhâ:14 beni anmak için namaz | 64. 3-Âl-i İmrân:190-191 göklerin yaratılışı | 65. 39-Zümer:9 bilenlerle bilmeyenler | 66. 20-Tâhâ:114 ilmimi artır | 67. 58-Mücâdele:11 ilim dereceleri | 68. 35-Fâtır:28 âlimler haşyet duyar | 69. 30-Rûm:22 diller ve renkler | 70. 88-Ğâşiye:17-20 deveye göğe bakmazlar mı

71. 29-Ankebût:45 namaz kötülükten alıkoyar | 72. 2-Bakara:45 sabır ve namazla yardım | 73. 20-Tâhâ:132 ailene namazı emret | 74. 107-Mâûn:4-5 namazından gafil | 75. 23-Mü'minûn:1-2 huşû duyan müminler | 76. 57-Hadîd:20 dünya oyun ve eğlence | 77. 3-Âl-i İmrân:185 her nefis ölümü tadacak | 78. 99-Zilzâl:7-8 zerre kadar hayır ve şer | 79. 21-Enbiyâ:35 şer ve hayırla imtihan | 80. 102-Tekâsür:1-2 çoklukla övünmek

81. 87-A'lâ:16-17 âhiret daha hayırlı | 82. 28-Kasas:77 âhireti iste | 83. 4-Nisâ:75 zayıf bırakılmışlar | 84. 22-Hac:39-40 zulme uğrayanlara izin | 85. 14-İbrâhim:42 Allah gafil değil | 86. 5-Mâide:32 bir cana kıymak | 87. 28-Kasas:5 zayıfları önder kılmak | 88. 2-Bakara:2 bu kitapta şüphe yok | 89. 17-İsrâ:9 en doğru yola iletir | 90. 54-Kamer:17 Kur'an'ı kolaylaştırdık

91. 10-Yûnus:57 göğüslere şifa | 92. 51-Zâriyât:58 rızık veren Allah | 93. 11-Hûd:6 her canlının rızkı | 94. 29-Ankebût:60 rızkını taşıyamaz | 95. 2-Bakara:216 hoşlanmadığınız hayırlı olabilir | 96. 33-Ahzâb:21 güzel örnek | 97. 21-Enbiyâ:107 âlemlere rahmet | 98. 68-Kalem:4 yüce ahlak | 99. 3-Âl-i İmrân:103 Allah'ın ipine sarılın

### ÖZEL GÜN AYETLERİ

- **Cuma:** 62-Cuma:9-10
- **Ramazan:** 2-Bakara:183 / 185, 97-Kadir:1-5
- **Kurban & Hac:** 22-Hac:37, 108-Kevser:1-3, 2-Bakara:197
- **Ramazan Bayramı:** 2-Bakara:185, 110-Nasr:1-3
- **Mevlid/Peygamber sevgisi:** 33-Ahzâb:21 / 56, 21-Enbiyâ:107, 68-Kalem:4
- **Kadir:** 97-Kadir:1-5
- **Miraç:** 17-İsrâ:1
- **Berat:** 44-Duhân:1-4
- **Muharrem/Aşure:** 89-Fecr:1-5, 2-Bakara:155-157
- **Zulüm/savaş/mazlum:** 4-Nisâ:75, 22-Hac:39-40, 5-Mâide:32, 14-İbrâhim:42, 28-Kasas:5
- **Afet/deprem:** 2-Bakara:155-157, 94-İnşirâh:5-6, 2-Bakara:286
- **Birlik/fitne:** 3-Âl-i İmrân:103, 8-Enfâl:46, 49-Hucurât:10

## ADIM 3: İÇERİK

Her seçenek için: harekeli Arapça metin, sade Türkçe meal (Diyanet mealine yakın, tırnak içinde), "{sure_no}-{Sure Adı} Sûresi: {ayet_no}" kaynak satırı, ve 1-2 cümlelik seçim gerekçesi.

Arapça metin hazırlarken:

- Vakf/tecvid işaretleri ayıklanır: U+0615 ve U+06D6–U+06ED aralığındaki karakterler silinir. Harekeler (fetha/damme/kesre/şedde/sükun/tenvin, U+0670, U+0656) KORUNUR.
- Her ayetin sonuna ayet numarası nişanı eklenir: `' ' + '﴿' + Arapça-Hint rakamı + '﴾'` (örn. 153 → " ﴿١٥٣﴾"). ASLA U+06DD kullanılmaz (bu fontta bozuk daire üretiyor). Başındaki numaranın alt satıra kopmasını engeller.
- Mealin başına "9-11:" gibi ayet no öneki YAZILMAZ (kaynak satırında zaten var).

**ÇOK UZUN AYETLER** (ör. 2-Bakara:282): tek karta okunaklı sığmaz. Böyle durumlarda ayet ANLAM BÜTÜNLÜĞÜNE GÖRE 2-3 parçaya bölünür ve options.json'da "parts" dizisi olarak verilir (her parça {arabic, meal}). Ayet-sonu nişanı SADECE son parçaya konur. Parts verilmezse kod gerekirse otomatik böler ama anlamlı bölme her zaman daha iyidir.

## ADIM 4: GÖRSELLERİ ÜRET

Tasarım Ali'nin geri bildirimleriyle netleşti (dikey 9:16, telefon tam ekran). Aşağıdaki dosyalar aynen oluşturulur ve çalıştırılır — tasarım yeniden yorumlanmaz.

### (a) Font kurulumu (her çalıştırmada gerekli)

```bash
cd /tmp && npm pack kfgqpc-uthmanic-script-hafs-regular && tar xzf kfgqpc-uthmanic-script-hafs-regular-*.tgz
sudo -n mkdir -p /usr/share/fonts/truetype/custom
sudo -n cp package/arabic.otf /usr/share/fonts/truetype/custom/KFGQPC-Uthmanic-Hafs.otf
sudo -n fc-cache -f
```
(fc-list | grep -i uthmanic ile doğrula)

### (b) Çalışma klasörü

`$HOME` değişkeninin gösterdiği dizin altında `ayet_kartlari` (örn. `$HOME/ayet_kartlari` — dikkat: bazı ortamlarda dosya yazma yeri ile bash'in `$HOME`'u farklı olabilir, `ls` ile doğrulanır).

**İçine `card_template.html` yazılır:**

```html
<!DOCTYPE html>
<html lang="tr"><head><meta charset="UTF-8">
<link href="https://fonts.googleapis.com/css2?family=Lora:ital,wght@0,400;0,600;1,400&display=swap" rel="stylesheet">
<style>
* { box-sizing: border-box; margin: 0; padding: 0; }
html, body { background: transparent; }
.card { width:1080px; min-height:1920px; padding:92px 74px 84px 74px;
  background:linear-gradient(160deg,#f6f1e4 0%,#eee5cf 100%); border:3px solid #8a6d1f;
  border-radius:24px; position:relative; display:flex; flex-direction:column;
  align-items:center; justify-content:center; font-family:'Lora',Georgia,serif; }
.card::before { content:""; position:absolute; top:14px; left:14px; right:14px; bottom:14px;
  border:1.5px solid #c9b177; border-radius:16px; pointer-events:none; }
.star { display:block; margin:0 auto 24px auto; }
.source { text-align:center; color:#8a6d1f; font-weight:700; font-size:40px; letter-spacing:0.3px; margin-bottom:56px; }
.part { text-align:center; color:#8a6d1f; font-weight:700; font-size:46px; letter-spacing:3px; margin-bottom:44px; }
.arabic { font-family:'KFGQPC Uthmanic Script HAFS',serif; direction:rtl; text-align:center;
  color:#1f3d2e; line-height:2.15; margin-bottom:60px; unicode-bidi:plaintext; width:100%; }
.divider { width:210px; height:1.5px; background:linear-gradient(90deg,transparent,#c9b177,transparent); margin:0 auto 56px auto; }
.meal { font-style:italic; text-align:center; color:#1f3d2e; font-size:46px; line-height:1.58; margin-bottom:56px; padding:0 4px; }
.ornament { text-align:center; color:#8a6d1f; font-size:26px; }
</style></head><body>
<div class="card" id="card">
<svg class="star" width="42" height="42" viewBox="0 0 100 100"><g fill="none" stroke="#8a6d1f" stroke-width="3">
<path d="M50 5 L58 40 L95 40 L64 58 L74 95 L50 72 L26 95 L36 58 L5 40 L42 40 Z"/></g></svg>
<div class="source" id="source">SOURCE</div>
<div class="part" id="part">PART</div>
<div class="arabic" id="arabic" style="font-size:50px;">ARABIC</div>
<div class="divider"></div>
<div class="meal" id="meal">MEAL</div>
<div class="ornament">✦</div>
</div></body></html>
```

**Renk paleti:** arka plan `#f6f1e4 → #eee5cf` (krem/bej degrade), çerçeve `#8a6d1f` (koyu altın) + iç çerçeve `#c9b177` (açık altın), Arapça/meal metin rengi `#1f3d2e` (koyu yeşil). Font: başlık/kaynak için Google Fonts "Lora" (serif), Arapça metin için "KFGQPC Uthmanic Script HAFS".

### (c) `options.json` yazılır. Format (parts opsiyonel):

```json
{ "A": {"source":"93-Duhâ Sûresi: 9-11","arabic":"...","meal":"“...”"},
  "B": {...}, "C": {"source":"2-Bakara Sûresi: 282","parts":[{"arabic":"...","meal":"..."},{...}]} }
```

### (d) `render.js` yazılır (AYNEN):

```javascript
const { chromium } = require('playwright');
const fs = require('fs'); const path = require('path');
const options = JSON.parse(fs.readFileSync(path.join(__dirname,'options.json'),'utf-8'));
const template = fs.readFileSync(path.join(__dirname,'card_template.html'),'utf-8');
const MIN_ARABIC = 46;
function splitArabic(text,n){const w=text.split(/\s+/).filter(Boolean);const p=Math.ceil(w.length/n);const o=[];for(let i=0;i<n;i++)o.push(w.slice(i*p,(i+1)*p).join(' '));return o.filter(Boolean);}
function splitMeal(text,n){const c=text.replace(/^[“"']\s*/,'').replace(/\s*[”"']$/,'');const s=c.split(/(?<=[.!?])\s+/).filter(Boolean);const t=c.length;const o=Array.from({length:n},()=>'');let i=0,a=0;for(const x of s){if(a>(t/n)*(i+1)&&i<n-1)i++;o[i]+=(o[i]?' ':'')+x;a+=x.length;}return o.filter(Boolean);}
async function renderCard(page,{source,part,arabic,meal},outPath){
  const html=template.replace('SOURCE',source).replace('PART',part||'').replace('ARABIC',arabic).replace('MEAL',meal);
  await page.setContent(html,{waitUntil:'networkidle'}); await page.waitForTimeout(400);
  const metrics=await page.evaluate((hasPart)=>{
    const card=document.getElementById('card'),ar=document.getElementById('arabic'),meal=document.getElementById('meal'),
      src=document.getElementById('source'),partEl=document.getElementById('part'),
      star=card.querySelector('.star'),divider=card.querySelector('.divider');
    if(!hasPart) partEl.style.display='none';
    const cs=getComputedStyle(card);
    const target=parseFloat(cs.minHeight)-parseFloat(cs.paddingTop)-parseFloat(cs.paddingBottom);
    const contentH=()=>Array.from(card.children).reduce((s,el)=>{ if(getComputedStyle(el).display==='none') return s;
      const r=el.getBoundingClientRect(); return s+r.height+parseFloat(getComputedStyle(el).marginBottom||0); },0);
    const B={ar:58,meal:46,src:40,part:46,mStar:24,mSrc:56,mPart:44,mAr:60,mDiv:56,mMeal:56};
    const CAP={ar:130,meal:92,src:76,part:68}; const M_CAP=2.05;
    const apply=(s)=>{ const arS=Math.min(CAP.ar,B.ar*s), srcS=Math.max(50,Math.min(CAP.src,B.src*s)),
      partS=Math.max(42,Math.min(CAP.part,B.part*s)), mealS=Math.min(CAP.meal,B.meal*s,arS*0.92), m=Math.min(M_CAP,s);
      ar.style.fontSize=arS+'px'; src.style.fontSize=srcS+'px'; partEl.style.fontSize=partS+'px'; meal.style.fontSize=mealS+'px';
      star.style.marginBottom=B.mStar*m+'px'; src.style.marginBottom=B.mSrc*m+'px'; partEl.style.marginBottom=B.mPart*m+'px';
      ar.style.marginBottom=B.mAr*m+'px'; divider.style.marginBottom=B.mDiv*m+'px'; meal.style.marginBottom=B.mMeal*m+'px';
      return {arS:Math.round(arS),mealS:Math.round(mealS),srcS:Math.round(srcS)}; };
    let best=0.4,info=apply(0.4);
    for(let s=0.4;s<=2.8;s=+(s+0.02).toFixed(2)){ const cur=apply(s); if(contentH()>target*0.975) break; best=s; info=cur; }
    info=apply(best);
    return {...info,doluluk:+(contentH()/target).toFixed(2)};
  },!!part);
  await page.waitForTimeout(200);
  if(outPath) await page.locator('#card').screenshot({path:outPath});
  return metrics; }
(async()=>{
  const browser=await chromium.launch({executablePath:'/opt/pw-browsers/chromium'});
  const page=await browser.newPage({viewport:{width:1200,height:3000},deviceScaleFactor:2});
  for(const key of Object.keys(options)){ const o=options[key];
    let parts=Array.isArray(o.parts)&&o.parts.length?o.parts:null;
    if(!parts){
      const test=await renderCard(page,{source:o.source,part:'',arabic:o.arabic,meal:o.meal},null);
      if(test.arS>=MIN_ARABIC){ await renderCard(page,{source:o.source,part:'',arabic:o.arabic,meal:o.meal},path.join(__dirname,`ayet_${key}.png`));
        console.log(`ayet_${key} tek kart arapça=${test.arS} meal=${test.mealS}`); continue; }
      for(let n=2;n<=4;n++){ const ars=splitArabic(o.arabic,n),mls=splitMeal(o.meal,n);
        const cand=ars.map((a,i)=>({arabic:a,meal:mls[i]||''}));
        const t=await renderCard(page,{source:o.source,part:`1 / ${n}`,arabic:cand[0].arabic,meal:cand[0].meal},null);
        if(t.arS>=MIN_ARABIC||n===4){ parts=cand; break; } } }
    const n=parts.length;
    for(let i=0;i<n;i++){ const out=path.join(__dirname,`ayet_${key}_${i+1}.png`);
      const m=await renderCard(page,{source:o.source,part:`${i+1} / ${n}`,arabic:parts[i].arabic,meal:parts[i].meal},out);
      console.log(`ayet_${key}_${i+1} arapça=${m.arS} meal=${m.mealS}`); } }
  await browser.close(); })();
```

### (e) `node render.js` çalıştırılır

Üretilen PNG'ler 1080x1920 (2x ölçek → 2160x3840) olmalı; kontrol edilir.

## ADIM 5: TESLİM

Üretilen tüm PNG'ler SendUserFile ile gönderilir (status: proactive). Parçalı bir ayet varsa parçalar sırayla ve birlikte gönderilir, hangi seçeneğin kaç parça olduğu belirtilir.

## ADIM 6: SON MESAJ (ÇOK ÖNEMLİ)

Son mesaj e-posta bildirimi olarak da gidiyor ve Ali bazen sadece bu metni okuyabiliyor. Bu yüzden 3 seçenek TAM yazılır: her biri için sure adı + ayet no + Türkçe mealin TAMAMI + kısa gerekçe. Görseli açamayan biri bile sadece bu metinle seçim yapabilmeli. Ayrıca yarın için tespit edilen dini gün/gündem durumu bir cümleyle belirtilir. Bu görev bir zamanlanmış görev (scheduled task) olarak, Ali uzaktayken çalışıyor; bu yüzden işin bitiminde PushNotification aracı da MUTLAKA kullanılır — routine_summary içine 3 seçeneğin kısa özeti (sure/ayet no + 1 cümle gerekçe) ve yarının gündem durumu yazılır, böylece Ali telefonuna bildirim ve e-posta olarak alır.

Tüm çıktılar Türkçe, saygılı ve sade olmalı. Görev Adım 6'da bitirilir, başka araç çağrısı yapılmaz. WebFetch KULLANILMAZ.
