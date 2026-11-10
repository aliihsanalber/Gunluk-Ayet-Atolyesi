//! Kur'an verisi (Arapça + meâller + sûre adları) ve kart metni üretimi.
//! Veri, derleme anında `data/qdata.json` dosyasından gömülür.

use serde::Deserialize;
use std::collections::BTreeMap;

const QDATA: &str = include_str!("../../data/qdata.json");
const EXTRA_MEALS: [(&str, &str); 18] = [
    ("elmalili", include_str!("../../meals/elmalili.json")),
    ("feyzul", include_str!("../../meals/feyzul.json")),
    ("ali_bulac", include_str!("../../meals/ali_bulac.json")),
    ("yasar", include_str!("../../meals/yasar.json")),
    ("aparliyan", include_str!("../../meals/aparliyan.json")),
    ("ahmettekin", include_str!("../../meals/ahmettekin.json")),
    ("ahmetvarol", include_str!("../../meals/ahmetvarol.json")),
    ("diyanetislerieski", include_str!("../../meals/diyanetislerieski.json")),
    ("diyanetisleriyeni", include_str!("../../meals/diyanetisleriyeni.json")),
    ("kuranyolu", include_str!("../../meals/kuranyolu.json")),
    ("diyanetvakfi", include_str!("../../meals/diyanetvakfi.json")),
    ("elmaliliorj", include_str!("../../meals/elmaliliorj.json")),
    ("hayrat", include_str!("../../meals/hayrat.json")),
    ("ihsanaktas", include_str!("../../meals/ihsanaktas.json")),
    ("mahmutkisa", include_str!("../../meals/mahmutkisa.json")),
    ("omernasuhi", include_str!("../../meals/omernasuhi.json")),
    ("syildirim", include_str!("../../meals/syildirim.json")),
    ("spiris", include_str!("../../meals/spiris.json")),
];

#[derive(Deserialize)]
pub struct Meal {
    pub tr: Vec<Vec<String>>,
    pub name: String,
    #[allow(dead_code)]
    pub author: String,
}

#[derive(Deserialize)]
pub struct Quran {
    pub names: Vec<String>,
    pub counts: Vec<usize>,
    pub ar: Vec<Vec<String>>,
    pub meals: BTreeMap<String, Meal>,
    #[serde(rename = "defaultMeal")]
    pub default_meal: String,
    pub picks: Vec<[usize; 3]>,
}

impl Quran {
    pub fn load() -> Quran {
        let mut q: Quran = serde_json::from_str(QDATA).expect("qdata.json okunamadı");
        // Varsayılan dışındaki meâller meals/ klasöründe ayrı dosyalardır.
        // Yeni meâl eklenince buraya da bir satır eklenir.
        for (key, json) in EXTRA_MEALS {
            q.meals.insert(key.to_string(), serde_json::from_str(json).expect("meal dosyası okunamadı"));
        }
        q
    }

    /// Sûre numarası (1-114) ya da (aksan/büyük-küçük harf duyarsız) sûre adı.
    pub fn find_sure(&self, q: &str) -> Option<usize> {
        if let Ok(n) = q.trim().parse::<usize>() {
            return (1..=114).contains(&n).then_some(n);
        }
        let key = norm(q.trim_end_matches(" sûresi").trim_end_matches(" suresi"));
        self.names.iter().position(|n| norm(n) == key).map(|i| i + 1)
    }

    pub fn sure_name(&self, s: usize) -> &str {
        &self.names[s - 1]
    }

    pub fn ayah_ar(&self, s: usize, v: usize) -> String {
        fix_tanwin_alef(self.ar[s - 1][v - 1].trim())
            .chars()
            .map(|c| match c {
                // Hafs fontu açık tanvin glifleri (U+08F0–08F2) içermez; standart tanvine eşle
                '\u{08F0}' => '\u{064B}',
                '\u{08F1}' => '\u{064C}',
                '\u{08F2}' => '\u{064D}',
                c => c,
            })
            .collect()
    }

    pub fn ayah_tr(&self, meal: &str, s: usize, v: usize) -> String {
        self.meals[meal].tr[s - 1][v - 1].trim().to_string()
    }

    pub fn ref_label(&self, s: usize, a1: usize, a2: usize) -> String {
        let r = if a1 == a2 { a1.to_string() } else { format!("{a1}-{a2}") };
        format!("{s}-{} Sûresi: {r}", self.sure_name(s))
    }
}

/// Kaynak veride açık tanvin (U+08F0–08F2) ile sondaki elif arasında boşluk kalmış
/// ("يُسۡرࣰ ا"); elif kelimeye bitiştirilir.
fn fix_tanwin_alef(t: &str) -> String {
    let mut out = String::with_capacity(t.len());
    let mut prev_open_tanwin = false;
    let mut chars = t.chars().peekable();
    while let Some(c) = chars.next() {
        if c == ' ' && prev_open_tanwin && chars.peek() == Some(&'ا') {
            let mut look = chars.clone();
            look.next();
            if look.peek().is_none_or(|n| n.is_whitespace()) {
                prev_open_tanwin = false;
                continue;
            }
        }
        prev_open_tanwin = ('\u{08F0}'..='\u{08F2}').contains(&c);
        out.push(c);
    }
    out
}

/// Türkçe harf/aksan sadeleştirme (arama ve sûre adı eşleştirme).
pub fn norm(s: &str) -> String {
    s.chars()
        .flat_map(|c| c.to_lowercase())
        .filter(|c| !matches!(c, '\'' | '’' | '`' | '\u{307}'))
        .map(|c| match c {
            'î' | 'í' | 'ı' => 'i',
            'â' | 'á' | 'ā' => 'a',
            'û' | 'ū' | 'ü' => 'u',
            'ö' => 'o',
            'ş' => 's',
            'ç' => 'c',
            'ğ' => 'g',
            c => c,
        })
        .collect()
}

pub fn arabic_num(n: usize) -> String {
    const D: [char; 10] = ['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'];
    n.to_string().chars().map(|c| D[c.to_digit(10).unwrap() as usize]).collect()
}

/// "5. Meal metni." — ayet no + sonunda noktalama garantisi.
pub fn meal_sentence(v: usize, txt: &str) -> String {
    let ends = txt.chars().last().is_some_and(|c| ".!?…”\"".contains(c));
    format!("{v}. {txt}{}", if ends { "" } else { "." })
}

/// Karttaki ayet gösterimi: "ayet ﴿٥﴾"
pub fn ayah_with_marker(q: &Quran, s: usize, v: usize) -> String {
    format!("{} ﴿{}﴾", q.ayah_ar(s, v), arabic_num(v))
}

/// Bir aralıktaki ayetlerin (numarasız) meâl metinleri.
pub fn verse_texts(q: &Quran, meal: &str, s: usize, a1: usize, a2: usize) -> Vec<String> {
    (a1..=a2).map(|v| q.ayah_tr(meal, s, v)).collect()
}

/// `texts[i]` = a1+i. ayetin (düzenlenmiş olabilecek) meâl metni.
pub fn compose_range(q: &Quran, texts: &[String], s: usize, a1: usize, a2: usize) -> (String, String, String) {
    let ar: Vec<String> = (a1..=a2).map(|v| ayah_with_marker(q, s, v)).collect();
    let tr: Vec<String> = (a1..=a2).map(|v| meal_sentence(v, texts[v - a1].trim())).collect();
    (q.ref_label(s, a1, a2), ar.join(" "), format!("“{}”", tr.join(" ")))
}

fn is_waqf(c: char) -> bool {
    ('\u{06D6}'..='\u{06DC}').contains(&c) || c == '\u{06D8}'
}

/// Uzun tek ayeti, durak (vakf) işaretine en yakın orta noktadan ikiye böler.
pub fn split_arabic_by_waqf(ar: &str) -> (String, String) {
    let chars: Vec<char> = ar.trim().chars().collect();
    // sondaki ﴿N﴾ işareti
    let mut marker = String::new();
    let mut body_end = chars.len();
    if chars.last() == Some(&'﴾') {
        if let Some(open) = chars.iter().rposition(|&c| c == '﴿') {
            marker = format!(" {}", chars[open..].iter().collect::<String>());
            body_end = open;
        }
    }
    let body: Vec<char> = chars[..body_end].iter().collect::<String>().trim().chars().collect();
    let mid = body.len() as f32 / 2.0;
    let pos: Vec<usize> = body.iter().enumerate().filter(|(_, &c)| is_waqf(c)).map(|(i, _)| i).collect();
    let cut = if let Some(&p) = pos
        .iter()
        .min_by(|&&a, &&b| (a as f32 - mid).abs().partial_cmp(&(b as f32 - mid).abs()).unwrap())
    {
        p + 1
    } else {
        let s: String = body.iter().collect();
        let words: Vec<&str> = s.split_whitespace().collect();
        let half = words.len().div_ceil(2);
        words[..half].join(" ").chars().count()
    };
    let p1: String = body[..cut].iter().collect::<String>().trim().to_string();
    let p2 = format!("{}{}", body[cut..].iter().collect::<String>().trim(), marker);
    (p1, p2)
}

/// Meâli cümle sınırından kabaca ikiye böler.
pub fn split_meal_by_sentence(meal: &str) -> (String, String) {
    let mut c = meal
        .trim_start_matches(|c: char| "“\"' \t\n".contains(c))
        .trim_end_matches(|c: char| "”\"' \t\n".contains(c))
        .to_string();
    // başta "12. " gibi ayet numarası
    let mut lead = String::new();
    let digits = c.chars().take_while(|c| c.is_ascii_digit()).count();
    if digits > 0 && c[digits..].starts_with('.') {
        let rest = c[digits + 1..].trim_start();
        lead = c[..c.len() - rest.len()].to_string();
        c = rest.to_string();
    }
    // [.!?…] sonrası boşlukta cümle sonu
    let mut sents: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut it = c.chars().peekable();
    while let Some(ch) = it.next() {
        cur.push(ch);
        if ".!?…".contains(ch) && it.peek().is_some_and(|n| n.is_whitespace()) {
            sents.push(std::mem::take(&mut cur).trim().to_string());
            while it.peek().is_some_and(|n| n.is_whitespace()) {
                it.next();
            }
        }
    }
    if !cur.trim().is_empty() {
        sents.push(cur.trim().to_string());
    }
    if sents.len() < 2 {
        let words: Vec<&str> = c.split_whitespace().collect();
        let h = words.len().div_ceil(2);
        return (format!("“{lead}{}…”", words[..h].join(" ")), format!("“…{}”", words[h..].join(" ")));
    }
    let total = c.chars().count() as f32;
    let (mut acc, mut i1, mut i2) = (0.0f32, Vec::new(), Vec::new());
    for s in sents {
        if acc < total / 2.0 {
            acc += s.chars().count() as f32;
            i1.push(s);
        } else {
            i2.push(s);
        }
    }
    if i2.is_empty() {
        i2.push(i1.pop().unwrap());
    }
    (format!("“{lead}{}”", i1.join(" ")), format!("“{}”", i2.join(" ")))
}

/// Düzenleme dosyası: her ayet "N: meâl metni" biçiminde (devam satırları eklenir).
pub fn to_edit_file(a1: usize, texts: &[String]) -> String {
    let mut o = String::from("# Meâl metinlerini düzenleyin; her ayet \"NUMARA: metin\" biçimindedir.\n# '#' ile başlayan satırlar yok sayılır. Kaydedip kapatın.\n\n");
    for (i, t) in texts.iter().enumerate() {
        o.push_str(&format!("{}: {}\n\n", a1 + i, t));
    }
    o
}

/// `to_edit_file` çıktısını okur; dosyada olmayan ayetler `texts`'teki hâliyle kalır.
pub fn apply_edit_file(a1: usize, texts: &mut [String], content: &str) {
    let mut cur: Option<usize> = None;
    let mut buf: Vec<String> = Vec::new();
    let flush = |cur: Option<usize>, buf: &mut Vec<String>, texts: &mut [String]| {
        if let Some(i) = cur {
            let t = buf.join(" ").trim().to_string();
            if !t.is_empty() {
                texts[i] = t;
            }
        }
        buf.clear();
    };
    for line in content.lines() {
        if line.trim_start().starts_with('#') {
            continue;
        }
        let head = line.split_once(':').and_then(|(n, rest)| n.trim().parse::<usize>().ok().map(|n| (n, rest)));
        match head {
            Some((n, rest)) if n >= a1 && n - a1 < texts.len() => {
                flush(cur, &mut buf, texts);
                cur = Some(n - a1);
                buf.push(rest.trim().to_string());
            }
            _ => buf.push(line.trim().to_string()),
        }
    }
    flush(cur, &mut buf, texts);
}

#[cfg(test)]
mod tests {
    #[test]
    fn duzenleme_dosyasi_gidis_donus() {
        let mut t = vec!["bir".to_string(), "iki".to_string()];
        let f = super::to_edit_file(5, &t).replace("iki", "ikinci\ndevam");
        super::apply_edit_file(5, &mut t, &f);
        assert_eq!(t, vec!["bir".to_string(), "ikinci devam".to_string()]);
    }

    use super::*;

    #[test]
    fn veri_tam() {
        let q = Quran::load();
        assert_eq!(q.counts.iter().sum::<usize>(), 6236);
        assert_eq!(q.names.len(), 114);
        assert_eq!(q.find_sure("94"), Some(94));
        assert_eq!(q.find_sure("Bakara"), Some(2));
        assert_eq!(q.find_sure("inşirah"), Some(94));
        assert_eq!(q.find_sure("115"), None);
    }

    #[test]
    fn etiket_ve_numara() {
        let q = Quran::load();
        assert_eq!(q.ref_label(94, 5, 6), "94-İnşirâh Sûresi: 5-6");
        assert_eq!(arabic_num(158), "١٥٨");
        assert_eq!(meal_sentence(3, "abc"), "3. abc.");
        assert_eq!(meal_sentence(3, "abc!"), "3. abc!");
    }

    #[test]
    fn tanvin_elif_birlesir() {
        let q = Quran::load();
        assert!(!q.ayah_ar(94, 6).contains("\u{064B} \u{0627}"));
        assert!(q.ayah_ar(94, 6).contains("\u{064B}\u{0627}"));
    }

    #[test]
    fn meal_bolme() {
        let (a, b) = split_meal_by_sentence("“1. Bir cümle. İkinci cümle. Üçüncü cümle burada.”");
        assert!(a.starts_with("“1. Bir") && a.ends_with('”'));
        assert!(b.starts_with('“') && b.contains("Üçüncü"));
        let (a, b) = split_meal_by_sentence("“tek cümle ama uzun bir metin”");
        assert!(a.ends_with("…”") && b.starts_with("“…"));
    }

    #[test]
    fn arapca_bolme_isaret_korunur() {
        let q = Quran::load();
        let ar = ayah_with_marker(&q, 7, 158);
        let (p1, p2) = split_arabic_by_waqf(&ar);
        assert!(!p1.is_empty() && p2.ends_with("﴿١٥٨﴾"));
    }
}
