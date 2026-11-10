//! Günün Ayeti Atölyesi — komut satırı sürümü (Rust).
//! Ayet + meâl kartlarını (1080×1920 PNG) tarayıcı olmadan üretir.

mod card;
mod data;

use card::{build_pages, Frame, Renderer, SplitMode, Style_};
use data::{norm, Quran};
use std::path::PathBuf;

const YARDIM: &str = "\
Günün Ayeti Atölyesi (Rust)

KULLANIM
  ayet kart <sûre> <ayet|başlangıç-bitiş> [seçenekler]   ayet kartı(ları) üret (PNG)
  ayet gunun [seçenekler]                                 öne çıkan ayetlerden bugünkünü üret
  ayet sureler                                            114 sûreyi ve ayet sayılarını listele
  ayet mealler                                            kullanılabilir meâller
  ayet ara <kelime> [--meal ANAHTAR]                      meâllerde ara
  ayet yardim

  <sûre>: numara (94) veya ad (inşirah, bakara)

SEÇENEKLER (kart / gunun)
  --meal ANAHTAR       diyanet (varsayılan) | elmalili | feyzul | ali_bulac
  --cerceve TUR        suslu (varsayılan) | sade | yok
  --bolme MOD          otomatik (varsayılan) | kapali | zorla
  --besmelesiz         kartlara besmele ekleme
  --zemin #RRGGBB      zemin rengi        (varsayılan #F4ECE0)
  --yazi #RRGGBB       ayet+meâl rengi    (varsayılan #234B32)
  --baslik #RRGGBB     başlık rengi       (varsayılan #8A6E1A)
  --cikti KLASOR       çıktı klasörü      (varsayılan .)
";

fn hata(m: &str) -> ! {
    eprintln!("hata: {m}\n\n`ayet yardim` ile kullanımı görün.");
    std::process::exit(2)
}

fn hex(s: &str) -> [u8; 3] {
    let h = s.trim_start_matches('#');
    if h.len() != 6 {
        hata(&format!("geçersiz renk: {s}"));
    }
    let p = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).unwrap_or_else(|_| hata(&format!("geçersiz renk: {s}")));
    [p(0), p(2), p(4)]
}

struct Opts {
    meal: Option<String>,
    style: Style_,
    mode: SplitMode,
    besmele: bool,
    out: PathBuf,
}

/// Seçenekleri ayıklar, konumsal argümanları döndürür.
fn parse_opts(args: &[String]) -> (Vec<String>, Opts) {
    let mut o = Opts { meal: None, style: Style_::default(), mode: SplitMode::Auto, besmele: true, out: PathBuf::from(".") };
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

fn uret(q: &Quran, s: usize, a1: usize, a2: usize, o: &Opts) {
    let meal = o.meal.clone().unwrap_or_else(|| q.default_meal.clone());
    if !q.meals.contains_key(&meal) {
        hata(&format!("bilinmeyen meâl: {meal} (`ayet mealler`)"));
    }
    std::fs::create_dir_all(&o.out).unwrap_or_else(|e| hata(&format!("klasör oluşturulamadı: {e}")));
    let mut r = Renderer::new();
    let pages = build_pages(&mut r, q, &meal, s, a1, a2, o.besmele, o.mode, o.style.frame);
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
            if pos.len() != 2 {
                hata("kullanım: ayet kart <sûre> <ayet|başlangıç-bitiş>");
            }
            let s = q.find_sure(&pos[0]).unwrap_or_else(|| hata(&format!("sûre bulunamadı: {}", pos[0])));
            let (a1, a2) = parse_range(&q, s, &pos[1]);
            uret(&q, s, a1, a2, &o);
        }
        "gunun" => {
            let (_, o) = parse_opts(rest);
            let days = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() / 86400).unwrap_or(0) as usize;
            let [s, a1, a2] = q.picks[days % q.picks.len()];
            uret(&q, s, a1, a2, &o);
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
