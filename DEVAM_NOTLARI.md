# Devam Notları (oturum devri)

Bu dosya, bir oturumda yapılanları ve bekleyen işleri yeni oturuma aktarmak için yazıldı.
Yeni oturumda önce bunu okuyun.

- **Dal:** `claude/daily-verse-workshop-rust-7ci8iy` (GitHub'a push edildi; `main`'e **birleştirilmedi**, PR açılmadı)
- **Canlı site:** https://aliihsanalber.github.io/Gunluk-Ayet-Atolyesi/ (GitHub Pages; büyük ihtimalle `main`'den yayınlanıyor, bu yüzden aşağıdaki değişiklikler dal birleşene kadar sitede görünmez)
- **Kullanıcı:** Türkçe konuşuyor. Her akşam telefondan (Chrome) ayet seçip düzenleyip WhatsApp'ta paylaşıyor. **Asıl kullandığı araç web sürümü**; Rust sürümü yedek/ek.

---

## 1. Yapılanlar

### 1.1 Rust sürümü (`ayet-rs/`)
Web aracının kart mantığının Rust'a taşınmış, tarayıcısız hâli. Komut satırı + yerel web arayüzü.

- `cd ayet-rs && cargo build --release && ./target/release/ayet web` → http://localhost:8792
- Komutlar: `kart <sûre> <ayet|bas-bit>` (`94:5-6` kısa yazımı da olur), `gunun`, `sureler`, `mealler`, `ara`, `web`, `yardim`
- Meâl düzenleme: `--duzenle`, `--meal-metni`, `--meal-dosya`
- `cosmic-text` (Arapça şekillendirme) + `tiny-skia` (çizim). Veri, Hafs fontu, çerçeve **derleme anında gömülür**.
- Dosyalar: `src/main.rs` (CLI), `src/data.rs` (veri + meâl bölme), `src/card.rs` (yerleşim + PNG), `src/web.rs` + `src/ui.html` (yerel arayüz)
- Başlık/meâl yazısı için **Lora yerine Liberation Serif** (`ayet-rs/fonts/`) kullanılıyor; ortamda Lora yoktu.
- Yeni veri yapısına uyarlandı: `qdata.json` yalnız Diyanet'i gömüyor, diğer 4 meâl `meals/*.json`'dan `include_str!` ile geliyor (`data.rs` içindeki `EXTRA_MEALS`). **Yeni meâl eklenirse bu listeye de satır eklenmeli.**
- Testler: `cargo test --release` → 7 test geçiyor.
- Kullanıcı Windows'ta Rust kurmaya çalıştı, Visual C++ ön koşulu çıktı; "Quick install via Visual Studio" ve "Desktop development with C++" önerildi. **Kurulumu tamamladığı doğrulanmadı.** Kullanıcı "şimdilik web sürümüyle devam" dedi.

### 1.2 Web sürümü v5 (`gunun-ayeti-atolyesi.src.html` → `index.html`)
Kullanıcının şikâyetlerine göre yapılanlar (hepsi Pixel 7 emülasyonuyla Playwright'ta test edildi, konsol hatası yok):

| İstek | Yapılan |
|---|---|
| Telefonda ayet numarası yazılamıyor | Başlangıç/bitiş ayeti artık **açılır menü** (`<select>`), sûreye göre dolar. Mobilde sûre ve ayet satırları ayrı, tam genişlik. |
| Gündem listesi istenmiyor | **Kaldırıldı** (HTML, CSS, JS, `build.js`). `data/news.json` dosyası duruyor ama kullanılmıyor, silinebilir. |
| Rastgele hep aynı ayetler | Eskiden yalnız ~50 `picks` listesinden seçiyordu. Artık **tüm Kur'an**'dan; "Elif-Lâm-Mîm" gibi harf ayetleri (≤5 harf) hariç. |
| Rastgele hafıza (1 ay) | `localStorage` anahtarı `gunun-ayeti-atolyesi.rastgele.v1`; **30 gün** (Ayarlar'dan değişir) tekrar önerilmez; "Geçmişi temizle" var. Yalnız o cihazın tarayıcısında saklanır. |
| Meâl düzenleme | Üst çubukta "✎ Meâli düzenle" → ayet başına textarea, "Orijinaline dön". Düzenlemeler (`EDITS`) yalnız oturum boyunca, seçili meâl için geçerli. Bölme/sığdırma düzenlenmiş metinle çalışır. |
| 10+ modern yazı tipi | **19 yazı tipi** (Lora, Playfair Display, Merriweather, Inter, DM Sans, Jakarta, Outfit, Sora, Figtree, Poppins, Montserrat, Roboto, Open Sans, Lato, Nunito, Raleway, Work Sans, Mulish, Source Sans) + "Meâl italik" anahtarı. Google Fonts'tan yüklenir (`@import`). |
| Besmele büyütme | Kaydırıcı + sayı kutusu + "Otomatik" düğmesi (`state.besmeleSize`). |
| Boyutları rakamla girme | Arapça, meâl, başlık, besmele, meâl sahibi, sayfa no için sayı kutuları. |
| Meâller | **5 meâl**: Diyanet, Elmalılı, Feyzü'l-Furkân, Ali Bulaç, **Yaşar Nuri Öztürk (yeni)**. |

Yapısal değişiklikler:
- Meâller artık `meals/<anahtar>.json` (+ `meals/index.json` kataloğu); **seçilince `fetch` ile yüklenir**. Sayfa 6,9 MB → ~2,5 MB. Yeni meâl eklemek = dosya koymak + `index.json`'a satır eklemek (sayfayı yeniden derlemek gerekmez; `mkdata.js` mevcut ek meâlleri katalogda korur).
- `mkdata.js` artık `data/qdata.json` (yalnız Diyanet gömülü, `catalog`, `picks`) + `meals/*.json` üretir. `build.js` `meals/`'ı `dist/`'e kopyalar.
- `sw.js` **v2**: sayfa önce ağdan (güncellemeler telefona hemen ulaşsın), çevrimdışıysa önbellekten. Eski SW "önce önbellek" idi ve güncellemeleri hiç ulaştırmıyordu.
- `serve.js` `.js` MIME türü düzeltildi.
- Arapça veri düzeltmesi (`mkdata.js` `fixAr`): kaynak veride açık tanvin (`U+08F0–08F2`) + ayrı `ا` vardı (ör. 94:6 `يُسۡرࣰ ا`); Hafs fontu bu işaretleri içermiyor. Elif bitiştirilip işaretler standart tanvine (`U+064B–064D`) eşlendi. Rust `data.rs` içinde de aynı düzeltme var.
- Feyzü'l-Furkân veride **428 ayet boş** (kaynak eksik). Bu ayetlerde Diyanet metni gösterilir, kartta "(eksik ayetlerde Diyanet)" yazar, toast uyarır.

### 1.3 Yayın akışı
```bash
node mkdata.js        # veri (gerekirse)
node build.js         # dist/gunun-ayeti-atolyesi.html + dist/meals/
cp dist/gunun-ayeti-atolyesi.html index.html   # sw.js, manifest.json, meals/ zaten kökte
node serve.js         # önizleme: http://localhost:8791 (PORT env ile değişir)
```
`index.html` kökte, `src.html`'in derlenmiş hâli. `src.html`'i değiştirince yeniden derleyip `index.html`'e kopyalamak gerekir.

README güncellendi (Rust bölümü, v5 notları, yayın akışı).

---

## 2. Bekleyen işler

### 2.1 kuranmeali.com meâlleri (KISMEN YAPILDI)
- **Eklendi (14 meâl):** Abdullah Parlıyan, Ahmet Tekin, Ahmet Varol, Diyanet Eski/Yeni, Kur'an Yolu, Diyanet Vakfı, Elmalılı Orijinal, Hayrat, İhsan Aktaş, Mahmut Kısa, Ömer Nasuhi Bilmen, Suat Yıldırım, Şaban Piriş. Scraper: `data/scrape_kuranmeali.py` (tek istek = ayetin tüm meâlleri; yeni meâl için aynı betik, anahtarlar sitedeki `value=` adları).
- Kullanıcının görsellerinde üstte kesik kalan (Gölpınarlı/Akgül?) ve Suat Yıldırım altındaki meâllerin seçili olup olmadığı belirsiz; sorulacak.
- Aşağıdaki not, ilk plan:
- Altyapı hazır: her meâl `meals/<anahtar>.json` = `{"name":..., "author":..., "tr":[[sûre1 ayetleri],[sûre2],...,[sûre114]]}` + `meals/index.json`'a `{key,name,author}` satırı.
- **Yapılacak:** siteyi aç, meâl listesini ve sayfa yapısını çıkar, bir scraper yaz (`data/scrape_feyzul.py` örnek), her meâlin 114 sûresini çek, dosyaları üret, kataloğa ekle, `ayet-rs/src/data.rs` `EXTRA_MEALS`'a da ekle.
- **Telif/koşul:** çoğu meâl telifli olabilir. Önce sitenin kullanım koşullarına/robots.txt'ye bak. Hepsini herkese açık repoya koymadan önce kullanıcıya sor (kendi kullanımı mı, herkese açık Pages mi).
- Boyut: her meâl ~1 MB JSON; çok meâlde repo büyür (gerekirse sıkıştırma/seçmeli ekleme düşünülebilir).

### 2.2 tevhidmeali.com/mushaf/2 Arapça fontu (EKLENMEMESİ ÖNERİLDİ)
- Font `Shaikh Hamdullah Mushaf`: "All rights reserved" telifi, 142 karakterlik alt küme; repoya konmadı.
- Sayfanın CSS'inden `font-family` / `@font-face` / font dosyası adresini bul.
- Lisansı kontrol et; uygunsa `assets/`'e koyup `build.js`/`src.html` içindeki `UthmanicHafs` gömmesini güncelle (şu an `KFGQPC Uthmanic Script HAFS`, `assets/HafsUthmanicScript-Regular.otf`).
- Dikkat: font değişirse sığdırma ölçüleri ve rozet (`﴿N﴾`) görünümü yeniden test edilmeli; Rust sürümü `ayet-rs/src/card.rs`'te font aile adı `KFGQPC Uthmanic Script HAFS` ve `../../fonts/HafsUthmanicScript-Regular.otf` yolunu kullanıyor.

### 2.3 Diğer
- **`main`'e birleştirme / PR:** Kullanıcı açıkça istemedikçe PR açma. Birleşmeden telefondaki site değişmez. Sorulmuştu, cevap gelmedi.
- **Yazı tiplerini telefonda gözle doğrula:** Test ortamında Google Fonts erişilemediği için Poppins/Montserrat vb. **gerçek glifleriyle görülmedi**; seçim ve CSS davranışı test edildi (font-family doğru uygulanıyor). Türkçe karakterler (ğ ş ı İ ö ç ü) tüm yazı tiplerinde doğru çıkıyor mu bak.
- **PNG dışa aktarma** (html-to-image, cdnjs) test ortamında denenmedi (cdnjs engelli). Yeni yazı tipleriyle **Paylaş → PNG** akışı telefonda denenmeli; html-to-image web fontlarını gömer.
- **94:6'nın son kartta doğru çıktığı** gözle son kez teyit edilmedi (veri düzeltmesi yapıldı, kod testi geçti).
- **Eski Gündem routine'i** hâlâ çalışıyor olabilir: https://claude.ai/code/routines/trig_01CxgPvBxjPxMuS9P7EW6J3P — artık gereksiz; kullanıcı durdurmalı (oradan durdurulamadı).
- Claude Artifact adresi (README'deki eski `claude.ai/code/artifact/...`) artık güncel değil; kullanıcı GitHub Pages kullanıyor.

---

## 3. Ortam ve ağ (önemli)

- Mevcut/önceki oturum "Default" ortamdaydı ve `kuranmeali.com`, `tevhidmeali.com` erişilemezdi (`403` / `ENOTFOUND`).
- Bu repo için **kişisel ortam oluşturuldu: `gunluk-ayet-atolyesi`** (`env_01PQ5vSXkn8aYppdUMtX3GFH`). İzinli adresler: `www.kuranmeali.com`, `kuranmeali.com`, `tevhidmeali.com`, `www.tevhidmeali.com`, `fonts.googleapis.com`, `fonts.gstatic.com`, `generativelanguage.googleapis.com`. Paket yöneticileri izinli.
- **Yeni oturumu bu ortamda başlatın** (ortam menüsünden "gunluk-ayet-atolyesi"). Aksi halde siteler yine engelli olur.
- Claude in Chrome / Remote Control **bağlanmadı** (bulut oturumundan bilgisayara erişilemedi). Gerekmezse kullanmayın; ağ izni yeterli.
- GitHub push: bu oturumun başında 403 vardı, kullanıcı erişimi düzeltince push çalıştı. Yeni oturumda tekrar sorun olursa https://claude.ai/connect-github adresinden bağlanması gerekir.

---

## 4. Yeni oturumda hızlı başlangıç

```bash
git fetch origin claude/daily-verse-workshop-rust-7ci8iy
git checkout claude/daily-verse-workshop-rust-7ci8iy
# önce ağ erişimini dene:
curl -sS -m 20 -o /dev/null -w "%{http_code}\n" https://www.kuranmeali.com/
curl -sS -m 20 -o /dev/null -w "%{http_code}\n" https://tevhidmeali.com/mushaf/2
```
Kodlama kuralı: değişiklikleri `claude/daily-verse-workshop-rust-7ci8iy` dalına commit/push et. PR'ı yalnız kullanıcı isterse aç.

Test notu: web arayüzü Playwright (Chromium önceden kurulu) ile test edildi: Pixel 7 emülasyonu, `serve.js`'i `PORT=8791` ile çalıştırıp `http://localhost:8791/`. Google Fonts ve cdnjs isteklerini `route.abort()` ile engelleyip geri kalan akışlar (ayet menüleri, rastgele+hafıza, meâl düzenleme, yazı tipi ızgarası, sayı kutuları, besmele, meâl yükleme) doğrulandı. Test betikleri repoda **yok** (oturum geçici klasöründeydi); gerekirse yeniden yazılır.

## 5. Dosya haritası

```
gunun-ayeti-atolyesi.src.html   ← ASIL KAYNAK (web sürümü)
index.html                      ← derlenmiş hâl (GitHub Pages)
build.js / mkdata.js / serve.js ← derleme, veri üretimi, yerel sunucu
sw.js / manifest.json           ← PWA (sw v2: ağdan önce)
meals/*.json, meals/index.json  ← meâller (lazy yüklenir)
data/qdata.json                 ← gömülü veri (Arapça + Diyanet + katalog + picks)
data/tr_*.json, ar_uthmani.json ← mkdata.js kaynakları
assets/, fonts/                 ← Hafs fontu, çerçeve, ikonlar
ayet-rs/                        ← Rust sürümü
DEVAM_NOTLARI.md                ← bu dosya
```
