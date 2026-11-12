# Devam Notları (oturum devri)

Bu dosya, bir oturumda yapılanları ve bekleyen işleri yeni oturuma aktarmak için yazıldı.
Yeni oturumda önce bunu okuyun.

- **Dal:** `claude/daily-verse-workshop-rust-7ci8iy`. On üç PR (#1–#13) `main`'e **birleştirildi**; tüm özellikler canlı sitede ve APK'da. Yeni işe başlarken dalı `main`'in son hâlinden yeniden başlat (`git fetch origin main && git checkout -B claude/daily-verse-workshop-rust-7ci8iy origin/main`).
- **Canlı site:** https://aliihsanalber.github.io/Gunluk-Ayet-Atolyesi/ (GitHub Pages, `main`'den yayınlanır). Depo **herkese açık (public)**; kullanıcı bunu bilerek kabul etti ("telif sorunu olmaz", meâller depoda).
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

### 1.4 Sonradan eklenenler (v5 sonrası, hepsi `main`'de)
- **Meâller (14 yeni, toplam 19):** Abdullah Parlıyan, Ahmet Tekin, Ahmet Varol, Diyanet Eski/Yeni, Kur'an Yolu, Diyanet Vakfı, Elmalılı Orijinal, Hayrat, İhsan Aktaş, Mahmut Kısa, Ömer Nasuhi Bilmen, Suat Yıldırım, Şaban Piriş (+ önceki 5). Scraper: `data/scrape_kuranmeali.py` (tek istek = ayetin tüm meâlleri; anahtarlar sitedeki `value=` adları; tüm meâller çekilebilir, yalnız seçilenler repoya yazıldı). Rust `EXTRA_MEALS` da güncel.
- **Meâl ön ek temizliği:** 10 meâlde "99-100." / "3, 4." gibi aralık ön ekleri vardı; temizlendi. Aynı metni taşıyan ardışık ayetler uygulamada grup sayılır (`mealGroup`/`mealLabel`): kartta metin bir kez, "99-100." etiketiyle.
- **Kart düzeni:** "Boşluğu sayfaya yay" (`data-spread`, varsayılan açık; ölçüm kartı `.measure` doğal yükseklikte kalır). "Meâli iki yana yasla" (`data-justify`). Çok ayetli kartta her ayet ayrı `.m-line` bloğu: numara satır başında, yaslıyken ayetin son satırı sola yaslı. Meâl metni ayetleri `\n` ile ayırır; uzun kartı bölme (`splitMealBySentence`) ayet sınırından böler.
- **Arapça yazı tipi:** Medine Mushafı/Hafs (varsayılan), Amiri Quran, Scheherazade New, Noto Naskh, Lateef (`AR_FONTS`). **Kendi font dosyanı yükle** (.ttf/.otf/.woff/.woff2): IndexedDB (`ayet-fontlar`) + blob `@font-face`; cihazdan çıkmaz, repoya girmez, PNG dışa aktarmada çalışır (telefonda doğrulandı).
- **Arapçayı iki yana yasla** (`data-arjustify`): ayetler inline akar. **Koruma** `arJustifyGuard`: yaslanmamış hâlde (son satır hariç) herhangi bir satır kutunun %78'inden azsa o kartta yaslama kapanır (`AR_JUSTIFY_MIN_FILL`, `data-arjoff`). Ölçüm için kart DOM'da olmalı (`render` içinde `appendChild` sonrası çağrılır).
- **Paylaşım hafızası (web + APK):** Ayarlar `localStorage`'da (`gunun-ayeti-atolyesi.v4`) hatırlanır. Rastgele geçmişi artık yalnız **paylaşılan** ayetleri tutar (`gunun-ayeti-atolyesi.paylasilan.v1`, `markShared()`; paylaş/indir/kopyala başarılıysa işaretlenir, Android paylaşım iptalinde işaretlenmez). Paylaşılan ayet `randDays` (30) gün rastgelede çıkmaz; paylaşılmayanlar tekrar çıkabilir. Açılışta en son **paylaşılan** ayet gösterilir (yoksa son bakılan, o da yoksa rastgele). Eski `rastgele.v1` anahtarı silinir.
- **Yaslama ince ayarı (`mealJustifyGuard`):** Meâl yaslıyken (ayetin son satırı hariç) bir satırda boşluk genişliğin %15'inden fazlaysa (`MJ_MAX_SLACK`) yazı en çok %8 (`MJ_MAX_SHRINK`, 0,5'lik adımlarla) küçülerek sonraki kelimenin yukarı çekilmesi denenir; yeterli iyileşme yoksa boyut korunur. Elle boyutlanmış sayfaya dokunmaz; Ayarlar'da "Yaslarken geniş boşlukları yazıyı hafifçe küçülterek gider" anahtarı (`justFit`, varsayılan açık).
- **Yaslama geri düşüşü:** `mealJustifyGuard` yazıyı küçültünce de boşluk eşiğin altına inmezse o kartta yaslama bırakılır, meâl ortalanır (`data-justify="0"`, boyut orijinal). Eşik %15: 40 örnek ayette Lora/Poppins ile ~%70 yaslı kalır, ~%30 ortalanır.
- **Ayarlar menüsü açılır gruplar:** `details.sgroup` — Meâl (açık), Arapça, Görünüm, Yazı boyutu ve yerleşim, Rastgele ayet ve kart bölme, Gelişmiş düzenleme. Açık/kapalı durum `gunun-ayeti-atolyesi.ayarGruplari` anahtarında hatırlanır. Tüm kontrol kimlikleri (id) korundu.
- **Meâl sahibi satırı:** Satır sabit konumlu (`.c-author`) olduğundan metin bloğunun alt dolgusu artırıldı (`.card-inner` padding-bottom: çerçevesiz 210, basit 260, süslü 300 px) ki yayma düzeni onunla yapışmasın.
- **Telefonda ayarlar alttan açılan sayfa (≤700 px):** Ayarlar ekranı kaplamaz; alttan %48 yükseklikte sayfa açılır, üstte kart önizlemesi (`body.settings-open`, üst çubuk ve ayet kontrolleri geçici gizlenir) görünür kalır, her ayar anında karta yansır. Masaüstünde eskisi gibi sağ panel.
- **Kısa ayette kartı kısalt (`state.compact`, varsayılan kapalı, "Yazı boyutu ve yerleşim"):** Açıkken kısa ayetlerde yayma uygulanmaz; başlık/besmele/ayet/meâl 34 px aralıkla yaklaşır, kart yüksekliği içeriğe göre 1350–1920 px olur (`computeCardH`, `--card-h`, `pg.cardH`). Süslü çerçeve artık iki katman (üst/alt yarı, `.ornate.o-top/.o-bot`, arka plan görseli tek `<style>` ile) olduğundan kart kısalınca köşe süsleri kırpılmaz. PNG çıktısı `el.offsetHeight` yüksekliğinde (genişlik 1080). Önizleme ölçeği `fitPreview` en yüksek karta göre.
- Telefonda doğrulananlar: PNG çıktısı (yeni yazı tipleriyle), 13:40-43 tek kartta sığıyor.

### 1.5 Android uygulaması (APK)
- **Yapı:** `android-app/` = Capacitor 6 sarmalayıcı (`appId com.aliihsanalber.ayetatolyesi`). `prepare-www.js` repo kökündeki `index.html`, `manifest.json`, `meals/` ve simgeleri `www/`'e kopyalar; `android/` Capacitor'ın ürettiği Gradle projesi (simgeler `assets/icon-512.png`'den). Eklentiler: `@capacitor/share`, `@capacitor/filesystem`.
- **Derleme:** `.github/workflows/android-apk.yml` (GitHub Actions, ubuntu + JDK17). `main`'e `index.html`, `meals/`, `android-app/**` değişikliği gelince (ya da elle `workflow_dispatch`) debug imzalı APK derler ve `apk-latest` sürümüne yükler. Her birleşmede APK kendiliğinden yenilenir (~5 dk).
- **İndirme:** https://github.com/aliihsanalber/Gunluk-Ayet-Atolyesi/releases/download/apk-latest/ayet-atolyesi.apk (debug imzalı: "bilinmeyen kaynaklar" izniyle kurulur; Play Store için ayrı imza gerekir).
- **Önemli hatalar ve dersler:** (1) `index.html`'de `<meta charset="utf-8">` yoktu; GitHub Pages karakter kodunu HTTP başlığında gönderdiği için web'de sorun yoktu ama WebView'da Türkçe bozuldu, Arapça içeren düzenli ifade betiği durdurdu, ekran boş kaldı. Düzeltildi; **APK'yı telefonda ya da charset başlığı göndermeyen yerel sunucuyla (`/tmp/.../nocs.js` benzeri) test etmeden teslim etme**. (2) WebView'da `navigator.share` yok: `isNative()` ise `nativeShare()` Filesystem(CACHE)+Share ile yerel paylaşım ekranını açar; paylaşım iptal edilirse "paylaşıldı" sayılmaz.
- **Sınırlar:** Arapça fontlar (Google Fonts) ve PNG aracı (html-to-image, cdnjs) internetten yüklenir; uygulama çevrimiçi çalışır. APK'nın telefonda tam doğrulanan kısmı: açılış, ayet/meâl, PNG paylaşımı (kullanıcı denedi). Servis çalışanı (`sw.js`) APK'ya kopyalanmıyor.
- **Hatırlatma:** Kullanıcı için kurulan tek seferlik "APK hatırlatması" çalıştı (trig_017NTQPQgUEwiJVFdjWisgER), tekrar gerekmez.

---

## 2. Bekleyen işler / açık konular

### 2.1 Meâller
- Kullanıcının görsellerinde kesik kalan seçimler belirsiz: Gölpınarlı / Akgül (listenin başı) ve Suat Yıldırım'ın altındakiler (Süleyman Ateş, Yaşar Nuri vb.). Kullanıcı "bu kadar meâl yeter" dedi; yeni istek gelirse `data/scrape_kuranmeali.py` ile eklenir (`meals/<anahtar>.json` + `meals/index.json` + Rust `EXTRA_MEALS`).
- Ön ek temizliğinden sonra birkaç yüz üst üste binen/farklı yazılmış aralıkta yineleme tam temizlenmemiş olabilir.
- Her meâl ~1 MB; `meals/` toplam ~23 MB. Sayfa yalnız seçilen meâli yükler.

### 2.2 Abay fontları (Shaikh Hamdullah Mushaf / KK_Abay / Amasya) — EKLENMEDİ
- Diyanet kuran sitesi, tevhidmeali ve kurankelimemeali bu fontları kullanıyor. Telif: "All rights reserved", Amasya'da "çoğaltmak için yazılı izin gerekir (göstermek/yazdırmak serbest)". **Repoya konmadı.**
- Çözüm: kullanıcı fontu kendisi indirip uygulamadaki **"Font dosyası yükle"** ile kullanır. Yazılı izin alınırsa doğrudan repoya konabilir.
- Rust sürümü `ayet-rs/src/card.rs` hâlâ yalnız Hafs'ı kullanıyor (Arapça font seçimi web'e özel).

### 2.3 Diğer
- **Eski Gündem routine'i** hâlâ çalışıyor olabilir: https://claude.ai/code/routines/trig_01CxgPvBxjPxMuS9P7EW6J3P — artık gereksiz; kullanıcı durdurmalı (oradan durdurulamadı).
- Rust sürümü: kullanıcı Windows'ta Rust kurmayı denedi (Visual C++ ön koşulu); tamamlandığı doğrulanmadı. "Şimdilik web sürümü" dedi.
- `data/news.json` kullanılmıyor, silinebilir.
- Yazı tipleri (19 meâl yazı tipi + 5 Arapça) Google Fonts'tan yüklenir; çevrimdışıyken yedek fontlara düşer.
- Yaslama eşiği (%78) kullanıcıya bırakıldı; telefonda sorun görülürse `AR_JUSTIFY_MIN_FILL` değiştirilir.
- PR kuralı: kullanıcı bu oturumda her seferinde PR'ın açılıp birleştirilmesini istedi/onayladı; yine de yeni işte sor.

---

## 3. Ortam ve ağ (önemli)

- "Default" ortamda `kuranmeali.com`, `tevhidmeali.com` erişilemezdi (`403`). Bu repo için kişisel ortam kullanılınca erişim çalıştı.
- Bu repo için **kişisel ortam oluşturuldu: `gunluk-ayet-atolyesi`** (`env_01PQ5vSXkn8aYppdUMtX3GFH`). İzinli adresler: `www.kuranmeali.com`, `kuranmeali.com`, `tevhidmeali.com`, `www.tevhidmeali.com`, `fonts.googleapis.com`, `fonts.gstatic.com`, `generativelanguage.googleapis.com`, `www.kurankelimemeali.com`, `kurankelimemeali.com`, `kuran.diyanet.gov.tr` (son üçü font sayfalarını incelemek için eklendi). Android derlemesi GitHub Actions üzerinde yapılır, bu ortamda Android SDK/Gradle yok. Paket yöneticileri izinli.
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
android-app/                    ← Android (Capacitor) projesi; android/ üretilmiş Gradle projesi, www/ derleme sırasında oluşur
.github/workflows/android-apk.yml ← APK derleme + apk-latest sürümü
data/scrape_kuranmeali.py       ← kuranmeali.com meâl kazıyıcı
DEVAM_NOTLARI.md                ← bu dosya
```
