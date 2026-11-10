//! Günün Ayeti Atölyesi — komut satırı sürümü (Rust).
//! Ayet + meâl kartlarını (1080×1920 PNG) tarayıcı olmadan üretir.

mod card;
mod data;
mod web;

use card::{build_pages, parse_hex, Frame, Renderer, SplitMode, Style_};
use data::{norm, Quran};
use std::path::PathBuf;

const YARDIM: &str = "\
Günün Ayeti Atölyesi (Rust)

KULLANIM
  ayet web [--port 8792]                  tarayıcıdan kullanılan arayüzü aç (önerilen)
  ayet kart <sûre> <ayet|bas-bit>         kart(lar) üret (PNG);  ör: ayet kart 94 5-6   veya   ayet kart 94:5-6
  ayet gunun                              öne çıkan ayetlerden bugünkünü üret
  ayet sureler | mealler                  sûre listesi | meâl listesi
  ayet ara <kelime> [--meal ANAHTAR]      meâllerde ara

  <sûre>: numara (94) veya ad (inşirah, bakara)

MEÂLİ DÜZENLEME (kart / gunun)
  --duzenle            meâli editörde (EDITOR/VISUAL, yoksa nano/vi) açar; kaydedince kart çıkar
  --meal-metni METIN   meâl metnini elle ver (numarasız; çok ayette sırayla tekrarla)
  --meal-dosya DOSYA   \"NUMARA: metin\" satırlı dosyadan oku (--duzenle'nin yazdığı biçim)

SEÇENEKLER
  --meal ANAHTAR       diyanet (varsayılan) | elmalili | feyzul | ali_bulac
  --cerceve TUR        suslu (varsayılan) | sade | yok
  --bolme MOD          otomatik (varsayılan) | kapali | zorla
  --besmelesiz         kartlara besmele ekleme
  --zemin #RRGGBB      zemin rengi        (varsayılan #F4ECE0)
  --yazi #RRGGBB       ayet+meâl rengi    (varsayılan #234B32)
  --baslik #RRGGBB     başlık rengi       (varsayılan #8A6E1A)
  --cikti KLASOR       çıktı klasörü      (varsayılan kartlar)
";

fn hata(m: &str) -> ! {
    eprintln!("hata: {m}\n\n`ayet yardim` ile kullanımı görün.");
    std::process::exit(2)
}

fn hex(s: &str) -> [u8; 3] {
    parse_hex(s).unwrap_or_else(|| hata(&format!("geçersiz renk: {s} (örnek: #F4ECE0)")))
}

struct Opts {
    meal: Option<String>,
    style: Style_,
    mode: SplitMode,
    besmele: bool,
    out: PathBuf,
    edit: bool,
    texts: Vec<String>,
    text_file: Option<PathBuf>,
}

/// Seçenekleri ayıklar, konumsal argümanları döndürür.
fn parse_opts(args: &[String]) -> (Vec<String>, Opts) {
    let mut o = Opts { meal: None, style: Style_::default(), mode: SplitMode::Auto, besmele: true, out: PathBuf::from("kartlar"), edit: false, texts: Vec::new(), text_file: None };
    let mut pos = Vec::new();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut val = |name: &str| it.next().cloned().unwrap_or_else(|| hata(&format!("{name} bir değer ister")));
        match a.as_str() {
            "--meal" => o.meal = Some(val("--meal")),
            "--cerceve" => {
                o.style.frame = match val("--cerceve").as_str() {
                    "suslu" => Frame::Ornate,
                    "sade" => Frame::Simple,
                    "yok" => Frame::None,
                    x => hata(&format!("bilinmeyen çerçeve: {x}")),
                }
            }
            "--bolme" => {
                o.mode = match val("--bolme").as_str() {
                    "otomatik" => SplitMode::Auto,
                    "kapali" => SplitMode::Off,
                    "zorla" => SplitMode::Force,
                    x => hata(&format!("bilinmeyen bölme modu: {x}")),
                }
            }
            "--besmelesiz" => o.besmele = false,
            "--duzenle" => o.edit = true,
            "--meal-metni" => o.texts.push(val("--meal-metni")),
            "--meal-dosya" => o.text_file = Some(PathBuf::from(val("--meal-dosya"))),
            "--zemin" => o.style.bg = hex(&val("--zemin")),
            "--yazi" => o.style.ink = hex(&val("--yazi")),
            "--baslik" => {
                let c = hex(&val("--baslik"));
                o.style.src_ink = c;
                o.style.rule = c;
            }
            "--cikti" => o.out = PathBuf::from(val("--cikti")),
            s if s.starts_with("--") => hata(&format!("bilinmeyen seçenek: {s}")),
            _ => pos.push(a.clone()),
        }
    }
    (pos, o)
}

fn parse_range(q: &Quran, s: usize, txt: &str) -> (usize, usize) {
    let n = q.counts[s - 1];
    let (a, b) = match txt.split_once('-') {
        Some((a, b)) => (a.trim().parse::<usize>(), b.trim().parse::<usize>()),
        None => (txt.trim().parse::<usize>(), txt.trim().parse::<usize>()),
    };
    match (a, b) {
        (Ok(a), Ok(b)) if 1 <= a && a <= b && b <= n => (a, b),
        _ => hata(&format!("{} sûresinde geçerli ayet aralığı 1-{n}", q.sure_name(s))),
    }
}

/// Meâl metinlerini belirler: kaynak meâl -> --meal-metni -> --meal-dosya -> --duzenle
fn meal_metinleri(q: &Quran, meal: &str, s: usize, a1: usize, a2: usize, o: &Opts) -> Vec<String> {
    let mut texts = data::verse_texts(q, meal, s, a1, a2);
    if !o.texts.is_empty() {
        if o.texts.len() > texts.len() {
            hata(&format!("{} ayet için {} --meal-metni verildi", texts.len(), o.texts.len()));
        }
        for (i, t) in o.texts.iter().enumerate() {
            texts[i] = t.clone();
        }
    }
    if let Some(f) = &o.text_file {
        let c = std::fs::read_to_string(f).unwrap_or_else(|e| hata(&format!("{}: {e}", f.display())));
        data::apply_edit_file(a1, &mut texts, &c);
    }
    if o.edit {
        let path = std::env::temp_dir().join(format!("ayet_meal_{s}_{a1}-{a2}.txt"));
        std::fs::write(&path, data::to_edit_file(a1, &texts)).unwrap_or_else(|e| hata(&format!("geçici dosya: {e}")));
        let ed = std::env::var("VISUAL").or_else(|_| std::env::var("EDITOR")).unwrap_or_else(|_| {
            ["nano", "vi"].iter().find(|e| std::process::Command::new("which").arg(e).output().is_ok_and(|o| o.status.success())).unwrap_or(&"vi").to_string()
        });
        let mut parts = ed.split_whitespace();
        let st = std::process::Command::new(parts.next().unwrap_or("vi")).args(parts).arg(&path).status();
        if !st.is_ok_and(|s| s.success()) {
            hata("editör çalıştırılamadı; --meal-metni veya --meal-dosya kullanabilirsiniz");
        }
        let c = std::fs::read_to_string(&path).unwrap_or_default();
        data::apply_edit_file(a1, &mut texts, &c);
        let _ = std::fs::remove_file(&path);
    }
    texts
}

fn uret(q: &Quran, s: usize, a1: usize, a2: usize, o: &Opts) {
    let meal = o.meal.clone().unwrap_or_else(|| q.default_meal.clone());
    if !q.meals.contains_key(&meal) {
        hata(&format!("bilinmeyen meâl: {meal} (`ayet mealler`)"));
    }
    let texts = meal_metinleri(q, &meal, s, a1, a2, o);
    std::fs::create_dir_all(&o.out).unwrap_or_else(|e| hata(&format!("klasör oluşturulamadı: {e}")));
    let mut r = Renderer::new();
    let pages = build_pages(&mut r, q, &meal, &texts, s, a1, a2, o.besmele, o.mode, o.style.frame);
    let ayet = if a1 == a2 { a1.to_string() } else { format!("{a1}-{a2}") };
    for (i, pg) in pages.iter().enumerate() {
        let name = if pages.len() > 1 { format!("ayet_{s}_{ayet}_{}.png", i + 1) } else { format!("ayet_{s}_{ayet}.png") };
        let path = o.out.join(name);
        r.render(pg, &o.style).save_png(&path).unwrap_or_else(|e| hata(&format!("PNG yazılamadı: {e}")));
        println!("{}  ({})", path.display(), pg.src);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first() else {
        print!("{YARDIM}");
        return;
    };
    let q = Quran::load();
    let rest = &args[1..];
    match cmd.as_str() {
        "kart" => {
            let (pos, o) = parse_opts(rest);
            let (sure, aralik) = match pos.as_slice() {
                [x] if x.contains(':') => x.split_once(':').map(|(a, b)| (a.to_string(), b.to_string())).unwrap(),
                [a, b] => (a.clone(), b.clone()),
                _ => hata("kullanım: ayet kart <sûre> <ayet|başlangıç-bitiş>   (veya 94:5-6)"),
            };
            let s = q.find_sure(&sure).unwrap_or_else(|| hata(&format!("sûre bulunamadı: {sure}")));
            let (a1, a2) = parse_range(&q, s, &aralik);
            uret(&q, s, a1, a2, &o);
        }
        "gunun" => {
            let (_, o) = parse_opts(rest);
            let days = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() / 86400).unwrap_or(0) as usize;
            let [s, a1, a2] = q.picks[days % q.picks.len()];
            uret(&q, s, a1, a2, &o);
        }
        "web" => {
            let mut port = 8792u16;
            if let Some(i) = rest.iter().position(|a| a == "--port") {
                port = rest.get(i + 1).and_then(|p| p.parse().ok()).unwrap_or_else(|| hata("--port bir sayı ister"));
            }
            web::serve(q, port);
        }
        "sureler" => {
            for (i, n) in q.names.iter().enumerate() {
                println!("{:>3}  {:<12} {:>3} ayet", i + 1, n, q.counts[i]);
            }
        }
        "mealler" => {
            for (k, m) in &q.meals {
                println!("{:<10} {}{}", k, m.name, if *k == q.default_meal { "  (varsayılan)" } else { "" });
            }
        }
        "ara" => {
            let (pos, o) = parse_opts(rest);
            let Some(kelime) = pos.first() else { hata("kullanım: ayet ara <kelime>") };
            let meal = o.meal.unwrap_or_else(|| q.default_meal.clone());
            let m = q.meals.get(&meal).unwrap_or_else(|| hata(&format!("bilinmeyen meâl: {meal}")));
            let key = norm(kelime);
            let mut n = 0;
            for (si, sure) in m.tr.iter().enumerate() {
                for (vi, t) in sure.iter().enumerate() {
                    if norm(t).contains(&key) {
                        n += 1;
                        if n <= 30 {
                            let t: String = t.chars().take(110).collect();
                            println!("{}:{}  {}", si + 1, vi + 1, t);
                        }
                    }
                }
            }
            println!("— {n} sonuç{}", if n > 30 { " (ilk 30 gösterildi)" } else { "" });
        }
        "yardim" | "yardım" | "--help" | "-h" => print!("{YARDIM}"),
        x => hata(&format!("bilinmeyen komut: {x}")),
    }
}
