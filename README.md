# Günün Ayeti Atölyesi

Ayet + meâl kartı hazırlama web aracı. Tek bir HTML dosyası olarak derlenir ve
**Claude Artifact** olarak yayımlanır. Sol tarafta her gün otomatik güncellenen
bir **Gündem** paneli vardır.

- **Yayınlanan sayfa:** https://claude.ai/code/artifact/1a48275a-dd46-463b-b9c4-8d6dc0871148
- **Gündem routine'i:** https://claude.ai/code/routines/trig_01CxgPvBxjPxMuS9P7EW6J3P
  (her gün 04:00 TR — gündemi tarar, ayetlerle eşleştirir, sayfayı aynı adrese
  yeniden yayımlar; **yalnız `<script id="news">` bloğunu** değiştirir)

---

## Gereksinimler

| | |
|---|---|
| **Node.js** | 18+ (kurulu: `node -v`) |
| **VS Code** | https://code.visualstudio.com |
| **Claude Code** | `npm i -g @anthropic-ai/claude-code` — sonra `claude` ile giriş yap |

VS Code için **Claude Code** eklentisi de kurabilirsin (Extensions → "Claude Code"),
ama şart değil; entegre terminalde `claude` komutu yeter.

---

## Başlangıç

```bash
# 1) bu klasörü VS Code'da aç
code "D:\Yapay Zeka\Claude\Ayet Paylaşımı\atolye"

# 2) bağımlılık (yalnız çerçeve script'i için; tek seferlik)
npm install

# 3) terminalde Claude Code'u başlat
claude
```

Artık Claude'a "sağ paneldeki renk kutularını büyüt", "yeni bir çerçeve ekle"
gibi isteklerde bulunabilirsin. Claude `gunun-ayeti-atolyesi.src.html` dosyasını
düzenler.

---

## Günlük iş akışı

### Önizleme (yerelde)

```bash
node build.js      # src -> dist/gunun-ayeti-atolyesi.html
node serve.js      # http://localhost:8791/
```

`build.js` çalışırken:
`@@FONT_B64@@`, `@@FRAME_B64@@`, `@@QDATA_JSON@@`, `@@NEWS_JSON@@`
yer tutucuları `assets/` ve `data/` içindekilerle doldurulur.

### Yayına alma (aynı Artifact adresine)

Claude Code oturumunda şunu iste:

> `dist/gunun-ayeti-atolyesi.html` dosyasını
> `https://claude.ai/code/artifact/1a48275a-dd46-463b-b9c4-8d6dc0871148`
> adresine yayınla (`capabilities: {"downloads": true}`).

**Önemli:** Yeni bir derleme yayınlamadan önce, routine'in güncellediği canlı
gündemi kaybetmemek için Claude'a şunu yaptır:

> Canlı artifact'i oku (`Artifact action:read`), içindeki
> `<script id="news" type="application/json">…</script>` bloğunu
> `data/news.json`'a kaydet. Sonra `node build.js` çalıştır ve yayınla.

(Alternatif: hiçbir şey yapma; routine ertesi sabah gündemi zaten düzeltir.)

---

## Sık yapılan işler

| İstek | Ne olur |
|---|---|
| Kur'an metnini yeniden üret | `node mkdata.js` → `data/qdata.json` |
| Başka bir süslü çerçeve dene | `assets/cerceve2.png`'i değiştir → `node make-frame.js` → `node build.js` |
| Gündem başlıklarını elle değiştir | `data/news.json`'ı düzenle → `node build.js` → yayınla |
| Routine'i durdur / ayarla | https://claude.ai/code/routines veya Claude Code'da `/schedule` |
| Meâli değiştir | `data/tr_diyanet.json` yerine başka bir çeviri koy, `mkdata.js`'i güncelle |

---

## Klasör yapısı

```
atolye/
  gunun-ayeti-atolyesi.src.html   ← ASIL KAYNAK (Claude bunu düzenler)
  build.js                        ← src + varlıklar -> dist/…html
  mkdata.js                       ← data/qdata.json üretir (Kur'an verisi)
  make-frame.js                   ← süslü çerçeveyi şeffaf köşeliye çevirir
  serve.js                        ← yerel önizleme (localhost:8791)
  package.json
  assets/
    HafsUthmanicScript-Regular.otf   Arapça font (KFGQPC Uthmani Hafs)
    cerceve2.png                     orijinal süslü çerçeve (krem zeminli)
    cerceve2-clear.png               şeffaf köşeli (build bunu gömer)
  data/
    qdata.json      tüm Kur'an: Arapça (QPC Uthmani Hafs) + meâl (Diyanet İşleri)
                    + sûre adları + "öne çıkan" ayet listesi (picks)
    news.json       sol Gündem paneli (routine bunun canlı sürümünü günceller)
    ar_uthmani.json / tr_diyanet.json   mkdata.js kaynakları (fawazahmed0/quran-api)
  dist/             ← build çıktısı (git'e girmez)
```

## Özellikler (v4)

- Aranabilir sûre seçici (tıkla → 1–114 kaydırmalı liste; yaz → canlı filtre)
- Elle ayet no / ardışık ayet bütünü
- **Çok kartlı model:** sığmayan ayet(ler) otomatik ayrı kartlara bölünür
  (ardışık ayetler → ayet başına kart; tek uzun ayet → durak/waqf işaretinden
  anlam bölmesi, 2. kartta besmele yok, sağ altta "1 / 2" numarası).
  Önizlemede yan yana; bir karta tıklayıp onu ayrı biçimlendirebilirsin.
- **Kart bazında** renk (ayet+meâl / başlık ayrı) ve font boyutu (Arapça / meâl /
  başlık ayrı). Çerçeve ve zemin rengi tüm kartlarda ortak.
- Çerçeve: yok / sade dikdörtgen / süslü (şeffaf köşeli)
- Ayet numarası düz süslü parantez `﴿…﴾` (daire yok)
- PNG dışa aktar + WhatsApp / Telegram / Instagram / X / Facebook
  (mobilde paylaşım ekranı; masaüstünde panoya kopyala + siteyi aç)
- Açık / koyu tema

## Git (isteğe bağlı)

```bash
git init
git add .
git commit -m "Günün Ayeti Atölyesi v4"
```
