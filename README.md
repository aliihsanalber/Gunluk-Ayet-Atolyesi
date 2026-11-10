# Günün Ayeti Atölyesi

Ayet + meâl kartı hazırlama web aracı. Tek bir HTML dosyası olarak derlenir ve
**Claude Artifact** olarak yayımlanır. Sol tarafta her gün otomatik güncellenen
bir **Gündem** paneli vardır.

- **Yayınlanan sayfa (telefonda kullanılan):** https://aliihsanalber.github.io/Gunluk-Ayet-Atolyesi/
  (GitHub Pages — repo kökündeki `index.html`, `meals/`, `sw.js`, `manifest.json`)
- Sol taraftaki **Gündem paneli kaldırıldı**; eski Gündem routine'i artık gereksizdir
  (https://claude.ai/code/routines/trig_01CxgPvBxjPxMuS9P7EW6J3P — durdurabilirsiniz).

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

`build.js` çalışırken `@@FONT_B64@@`, `@@FRAME_B64@@`, `@@QDATA_JSON@@`
yer tutucuları `assets/` ve `data/` içindekilerle doldurulur; `meals/*.json`
dosyaları `dist/meals/` içine kopyalanır.

**GitHub Pages'e yayın:** `node build.js` sonrası `dist/gunun-ayeti-atolyesi.html`
dosyasını repo kökündeki `index.html` olarak kopyalayın (`sw.js`, `manifest.json`
ve `meals/` zaten kökte durur) ve commit/push edin.

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
| Başka bir süslü çerçeve dene | `assets/cerceve2.png`'i değiştir → `node make-frame.js` → `node build.js` |
| Yeni meâl ekle | `meals/<anahtar>.json` dosyası koy (`{name, author, tr:[[...114 sûre]]}`) + `meals/index.json`'a satır ekle |
| Routine'i durdur / ayarla | https://claude.ai/code/routines veya Claude Code'da `/schedule` |
| Meâl listesini yeniden üret | `node mkdata.js` → `data/qdata.json` + `meals/*.json` |

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
    ar_uthmani.json / tr_diyanet.json   mkdata.js kaynakları (fawazahmed0/quran-api)
  dist/             ← build çıktısı (git'e girmez)
```

## Rust sürümü (`ayet-rs/`)

Aynı kartları **tarayıcı olmadan**, tek bir Rust programıyla PNG olarak üretir
(Arapça şekillendirme `cosmic-text`, çizim `tiny-skia`; Kur'an verisi, Hafs fontu
ve süslü çerçeve derleme anında gömülür → tek dosyalık çalıştırılabilir).

```bash
cd ayet-rs
cargo build --release
./target/release/ayet web              # → http://localhost:8792  (önerilen kullanım)
```

**`ayet web`** tarayıcıda açılan yerel arayüzdür: sûre/ayet seçimi, meâl kaynağı,
**ayet başına düzenlenebilir meâl metni** ("Orijinaline dön" düğmesiyle), çerçeve,
renkler, canlı önizleme ve PNG indirme ("Günün ayeti" / "Rastgele öne çıkan" dahil).
Kartlar Rust tarafında çizilir; ek bağımlılık yoktur.

Komut satırı:

```bash
./target/release/ayet kart 94:5-6                            # kartlar/ klasörüne yazar
./target/release/ayet kart 94 5-6 --duzenle                  # meâli editörde düzelt, sonra kart çıkar
./target/release/ayet kart 94:5 --meal-metni "Kendi meâl metnim"
./target/release/ayet kart 94:5-6 --meal-dosya meal.txt      # "5: metin" / "6: metin" satırları
./target/release/ayet kart 2 255 --cerceve sade --meal elmalili
./target/release/ayet kart 103 1-3 --cerceve yok --zemin '#1a1d24' --yazi '#e8dcc0'
./target/release/ayet gunun | ara sabır | sureler | mealler | yardim
cargo test --release
```

Web aracıyla aynı mantık: otomatik punto sığdırma, sığmayan ayetlerin ayrı
kartlara bölünmesi (ardışık ayetler → ayet başına kart; tek uzun ayet → durak
işaretinden ikiye), `1 / 2` numarası, çerçeve (yok/sade/süslü), renkler, 4 meâl.

## Yenilikler (v5)

- **Ayet seçimi açılır menü:** sûre seçince başlangıç/bitiş ayeti menüden seçilir (telefonda numara yazmak gerekmez)
- **Rastgele ayet:** tüm Kur'an'dan seçer (eskiden yalnız ~50 öne çıkan ayet); önerilen ayetler
  **30 gün** tekrar önerilmez (gün sayısı Ayarlar'dan değişir, geçmiş tarayıcıda saklanır, temizlenebilir)
- **Meâl düzenleme:** üst çubuktaki "✎ Meâli düzenle" ile ayet başına metin düzeltilir ("Orijinaline dön" var)
- **5 meâl** (Yaşar Nuri Öztürk eklendi); meâller `meals/` içinde ayrı dosya, seçilince yüklenir (sayfa 6,9 → 2,5 MB)
- **19 meâl yazı tipi** (Poppins, Montserrat, Roboto, Open Sans, Lato, Nunito, Raleway, Work Sans, Mulish, Source Sans, Inter, DM Sans… + serifler) ve italik açma/kapama
- **Besmele boyutu** ayarı; Arapça / meâl / başlık / besmele / meâl sahibi / sayfa no boyutları **rakamla** girilebilir
- Service worker artık sayfayı önce ağdan alır: yeni sürüm telefonda hemen görünür

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
