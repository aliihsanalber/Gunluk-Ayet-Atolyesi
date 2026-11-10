//! Kart yerleşimi + PNG çizimi (cosmic-text ile Arapça şekillendirme, tiny-skia ile çizim).

use cosmic_text::{
    fontdb, Align, Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, Style, SwashCache,
    Weight, Wrap,
};
use tiny_skia::{Color as SkColor, Paint, PathBuilder, Pixmap, PixmapPaint, Rect, Stroke, Transform};

pub const W: u32 = 1080;
pub const H: u32 = 1920;
const BESMELE: &str = "بِسْمِ ٱللَّهِ ٱلرَّحْمَٰنِ ٱلرَّحِيمِ";
const CRAMPED_AR: f32 = 52.0;

const HAFS: &[u8] = include_bytes!("../../fonts/HafsUthmanicScript-Regular.otf");
const SERIF_I: &[u8] = include_bytes!("../fonts/LiberationSerif-Italic.ttf");
const SERIF_BI: &[u8] = include_bytes!("../fonts/LiberationSerif-BoldItalic.ttf");
const SANS: &[u8] = include_bytes!("../fonts/LiberationSans-Regular.ttf");
const FRAME_PNG: &[u8] = include_bytes!("../../assets/cerceve2-clear.png");

const AR_FAMILY: &str = "KFGQPC Uthmanic Script HAFS";
const SERIF: &str = "Liberation Serif";
const SANS_FAMILY: &str = "Liberation Sans";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Frame {
    None,
    Simple,
    Ornate,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SplitMode {
    Auto,
    Off,
    Force,
}

pub type Rgb = [u8; 3];

#[derive(Clone)]
pub struct Style_ {
    pub bg: Rgb,
    pub ink: Rgb,
    pub src_ink: Rgb,
    pub rule: Rgb,
    pub frame: Frame,
}

impl Default for Style_ {
    fn default() -> Self {
        Style_ { bg: [0xF4, 0xEC, 0xE0], ink: [0x23, 0x4B, 0x32], src_ink: [0x8A, 0x6E, 0x1A], rule: [0x8A, 0x6E, 0x1A], frame: Frame::Ornate }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Page {
    pub src: String,
    pub arabic: String,
    pub meal: String,
    pub besmele: bool,
    pub chip: String,
    pub page_label: String,
    pub author: String,
}

pub struct Renderer {
    fs: FontSystem,
    cache: SwashCache,
}

struct Sizes {
    ar: f32,
    meal: f32,
    src: f32,
    besmele: f32,
}

fn clamp(x: f32, a: f32, b: f32) -> f32 {
    x.max(a).min(b)
}

/// Metin içinde ayet sonu rozeti ﴿N﴾ ayetten kopmasın diye araya bölünmez boşluk konur,
/// her ayet ayrı paragraf (satır) olur.
fn arabic_paragraphs(raw: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut rest = raw.trim();
    while let Some(end) = rest.find('﴾') {
        let end = end + '﴾'.len_utf8();
        out.push(rest[..end].trim().replace(" ﴿", "\u{00A0}﴿"));
        rest = rest[end..].trim();
    }
    if !rest.is_empty() {
        out.push(rest.to_string());
    }
    out.join("\n")
}

impl Renderer {
    pub fn new() -> Self {
        let sources = [HAFS, SERIF_I, SERIF_BI, SANS]
            .map(|b| fontdb::Source::Binary(std::sync::Arc::new(b.to_vec())));
        Renderer { fs: FontSystem::new_with_fonts(sources), cache: SwashCache::new() }
    }

    fn buffer(&mut self, text: &str, family: &str, size: f32, lh: f32, width: f32, style: Style, weight: Weight, align: Align) -> Buffer {
        let mut b = Buffer::new(&mut self.fs, Metrics::new(size, size * lh));
        b.set_wrap(Wrap::Word);
        b.set_size(Some(width), None);
        let attrs = Attrs::new().family(Family::Name(family)).style(style).weight(weight);
        b.set_text(text, &attrs, Shaping::Advanced, Some(align));
        b.shape_until_scroll(&mut self.fs, false);
        b
    }

    fn height(b: &Buffer) -> f32 {
        b.layout_runs().map(|r| r.line_height).sum()
    }

    fn make(&mut self, pg: &Page, sz: &Sizes, w: f32) -> Vec<(Buffer, f32)> {
        // (buffer, alt boşluk) — yukarıdan aşağı
        let mut v = Vec::new();
        v.push((self.buffer(&pg.src, SERIF, sz.src, 1.25, w, Style::Italic, Weight::BOLD, Align::Center), sz.src * 0.55));
        if pg.besmele {
            v.push((self.buffer(BESMELE, AR_FAMILY, sz.besmele, 1.5, w, Style::Normal, Weight::NORMAL, Align::Center), sz.besmele * 0.9));
        }
        v.push((self.buffer(&arabic_paragraphs(&pg.arabic), AR_FAMILY, sz.ar, 1.9, w, Style::Normal, Weight::NORMAL, Align::Center), 0.0));
        v.push((self.buffer(&pg.meal, SERIF, sz.meal, 1.62, w, Style::Italic, Weight::NORMAL, Align::Center), 0.0));
        v
    }

    /// Yığın yüksekliği: kaynak, besmele, arapça, ayraç, meâl.
    fn stack_height(&mut self, pg: &Page, sz: &Sizes, w: f32) -> f32 {
        let items = self.make(pg, sz, w);
        let n = items.len();
        let mut h = 0.0;
        for (i, (b, gap)) in items.iter().enumerate() {
            h += Self::height(b) + gap;
            if i == n - 2 {
                h += sep_block(sz.meal); // arapça ile meâl arası ayraç
            }
        }
        h
    }

    fn area(frame: Frame) -> (f32, f32, f32) {
        // (yatay iç boşluk, dikey iç boşluk) -> (kullanılabilir genişlik, yükseklik, üst ofset)
        let (px, py) = match frame {
            Frame::None => (132.0, 150.0),
            Frame::Simple => (148.0, 172.0),
            Frame::Ornate => (150.0, 292.0),
        };
        (W as f32 - 2.0 * px, H as f32 - 2.0 * py, py)
    }

    /// En büyük sığan Arapça punto (ikili arama). Dönüş: (boyutlar, taştı mı).
    fn autofit(&mut self, pg: &Page, frame: Frame) -> (Sizes, bool) {
        let (w, avail_h, _) = Self::area(frame);
        let n = pg.arabic.chars().filter(|c| !c.is_whitespace()).count() as f32;
        let ar_cap = clamp(76.0 + (n - 28.0) * 0.36, 76.0, 140.0);
        let meal_cap = clamp(34.0 + (n - 40.0) * 0.12, 34.0, 60.0);
        let sizes = |ar: f32| {
            let meal = clamp(ar * 0.60, 22.0, meal_cap);
            Sizes { ar, meal, src: clamp(meal * 0.94, 30.0, 60.0), besmele: clamp(meal * 0.98, 30.0, 58.0) }
        };
        let (mut lo, mut hi, mut best) = (28.0f32, ar_cap, 28.0f32);
        for _ in 0..22 {
            let mid = (lo + hi) / 2.0;
            let s = sizes(mid);
            if self.stack_height(pg, &s, w) <= avail_h * 0.94 {
                best = mid;
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let s = sizes(best.round());
        let over = self.stack_height(pg, &s, w) > avail_h + 1.0;
        (s, over)
    }

    /// (arapça punto, sıkışık mı)
    pub fn fit(&mut self, pg: &Page, frame: Frame) -> (f32, bool) {
        let (s, over) = self.autofit(pg, frame);
        (s.ar, s.ar < CRAMPED_AR || over)
    }

    pub fn render(&mut self, pg: &Page, st: &Style_) -> Pixmap {
        let mut pm = Pixmap::new(W, H).unwrap();
        pm.fill(SkColor::from_rgba8(st.bg[0], st.bg[1], st.bg[2], 255));
        draw_frame(&mut pm, st);

        let (sz, _) = self.autofit(pg, st.frame);
        let (w, avail_h, py) = Self::area(st.frame);
        let px = (W as f32 - w) / 2.0;
        let items = self.make(pg, &sz, w);
        let n = items.len();
        let total: f32 = items.iter().enumerate().map(|(i, (b, g))| {
            Self::height(b) + g + if i == n - 2 { sep_block(sz.meal) } else { 0.0 }
        }).sum();
        let mut y = py + (avail_h - total) / 2.0;

        for (i, (mut b, gap)) in items.into_iter().enumerate() {
            let is_src = i == 0;
            let is_besmele = pg.besmele && i == 1;
            let (col, op) = if is_src { (st.src_ink, 1.0) } else if is_besmele { (st.ink, 0.85) } else if i == n - 1 { (st.ink, 0.92) } else { (st.ink, 1.0) };
            self.blit(&mut pm, &mut b, px, y, col, op);
            y += Self::height(&b) + gap;
            if i == n - 2 {
                let m = sz.meal * 0.75;
                let sw = w * 0.22;
                let mut p = Paint::default();
                p.set_color(SkColor::from_rgba8(st.rule[0], st.rule[1], st.rule[2], 128));
                pm.fill_rect(Rect::from_xywh(W as f32 / 2.0 - sw / 2.0, y + m, sw, 2.0).unwrap(), &p, Transform::identity(), None);
                y += sep_block(sz.meal);
            }
        }

        // alt bilgiler (çerçeveye göre konum)
        let (chip_r, chip_b, author_b) = match st.frame {
            Frame::Ornate => (132.0, 128.0, 180.0),
            Frame::Simple => (96.0, 92.0, 140.0),
            Frame::None => (64.0, 58.0, 96.0),
        };
        if !pg.author.is_empty() {
            let mut b = self.buffer(&pg.author, SANS_FAMILY, 24.0, 1.3, W as f32, Style::Normal, Weight::NORMAL, Align::Center);
            let h = Self::height(&b);
            self.blit(&mut pm, &mut b, 0.0, H as f32 - author_b - h, st.ink, 0.55);
        }
        if !pg.page_label.is_empty() {
            let mut b = self.buffer(&pg.page_label, SANS_FAMILY, 26.0, 1.3, W as f32, Style::Normal, Weight::NORMAL, Align::Center);
            let h = Self::height(&b);
            let page_b = if st.frame == Frame::Simple { 78.0 } else { 52.0 };
            self.blit(&mut pm, &mut b, 0.0, H as f32 - page_b - h, st.ink, 0.38);
        }
        if !pg.chip.is_empty() {
            let mut b = self.buffer(&pg.chip, SERIF, 32.0, 1.3, 300.0, Style::Italic, Weight::BOLD, Align::Right);
            let h = Self::height(&b);
            self.blit(&mut pm, &mut b, W as f32 - chip_r - 300.0, H as f32 - chip_b - h, st.src_ink, 0.75);
        }
        pm
    }

    fn blit(&mut self, pm: &mut Pixmap, b: &mut Buffer, ox: f32, oy: f32, rgb: Rgb, opacity: f32) {
        let (ox, oy) = (ox.round() as i32, oy.round() as i32);
        let (pw, ph) = (pm.width() as i32, pm.height() as i32);
        let data = pm.data_mut();
        b.draw(&mut self.fs, &mut self.cache, Color::rgb(rgb[0], rgb[1], rgb[2]), |x, y, w, h, c| {
            let a = c.a() as f32 / 255.0 * opacity;
            if a <= 0.0 {
                return;
            }
            for yy in y + oy..y + oy + h as i32 {
                for xx in x + ox..x + ox + w as i32 {
                    if xx < 0 || yy < 0 || xx >= pw || yy >= ph {
                        continue;
                    }
                    let i = ((yy * pw + xx) * 4) as usize;
                    // zemin opak: src-over
                    for (k, ch) in [c.r(), c.g(), c.b()].into_iter().enumerate() {
                        data[i + k] = (ch as f32 * a + data[i + k] as f32 * (1.0 - a)).round() as u8;
                    }
                }
            }
        });
    }
}

fn sep_block(meal: f32) -> f32 {
    meal * 0.75 * 2.0 + 2.0
}

fn draw_frame(pm: &mut Pixmap, st: &Style_) {
    match st.frame {
        Frame::None => {}
        Frame::Simple => {
            let mut p = Paint::default();
            p.set_color(SkColor::from_rgba8(st.rule[0], st.rule[1], st.rule[2], 128));
            for (inset, width) in [(46.0f32, 2.5f32), (59.0, 1.0)] {
                let r = Rect::from_xywh(inset, inset, W as f32 - 2.0 * inset, H as f32 - 2.0 * inset).unwrap();
                let path = PathBuilder::from_rect(r);
                let stroke = Stroke { width, ..Default::default() };
                pm.stroke_path(&path, &p, &stroke, Transform::identity(), None);
            }
        }
        Frame::Ornate => {
            let img = Pixmap::decode_png(FRAME_PNG).expect("çerçeve PNG");
            pm.draw_pixmap(0, 0, img.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
        }
    }
}

/// Bir sayfayı iki karta böl (uzun tek ayet).
fn long_split(pg: &Page) -> Vec<Page> {
    let (a1, a2) = crate::data::split_arabic_by_waqf(&pg.arabic);
    let (m1, m2) = crate::data::split_meal_by_sentence(&pg.meal);
    vec![
        Page { arabic: a1, meal: m1, ..pg.clone() },
        Page { arabic: a2, meal: m2, besmele: false, ..pg.clone() },
    ]
}

fn renumber(pages: &mut [Page]) {
    let n = pages.len();
    for (i, p) in pages.iter_mut().enumerate() {
        if n < 2 {
            p.chip.clear();
            p.page_label.clear();
        } else {
            p.chip = format!("{} / {}", i + 1, n);
            p.page_label = format!("{} - {}", i + 1, n);
        }
    }
}

/// Seçilen ayet aralığını kartlara böler (web aracındaki `buildPages` ile aynı mantık).
pub fn build_pages(r: &mut Renderer, q: &crate::data::Quran, meal: &str, s: usize, a1: usize, a2: usize, besmele: bool, mode: SplitMode, frame: Frame) -> Vec<Page> {
    use crate::data::*;
    let author = q.meals[meal].name.clone();
    let (src, arabic, mealtxt) = compose_range(q, meal, s, a1, a2);
    let single = Page { src, arabic, meal: mealtxt, besmele, author: author.clone(), ..Default::default() };
    if mode == SplitMode::Off {
        return vec![single];
    }
    let (_, cramped) = r.fit(&single, frame);
    let mut pages: Vec<Page>;
    if mode == SplitMode::Auto && !cramped {
        return vec![single];
    } else if a2 > a1 {
        pages = Vec::new();
        for v in a1..=a2 {
            let pv = Page {
                src: q.ref_label(s, v, v),
                arabic: ayah_with_marker(q, s, v),
                meal: format!("“{}”", meal_sentence(v, &q.ayah_tr(meal, s, v))),
                besmele: v == a1 && besmele,
                author: author.clone(),
                ..Default::default()
            };
            let (_, c) = r.fit(&pv, frame);
            if c {
                pages.extend(long_split(&pv));
            } else {
                pages.push(pv);
            }
        }
    } else {
        pages = long_split(&single);
    }
    renumber(&mut pages);
    pages
}
