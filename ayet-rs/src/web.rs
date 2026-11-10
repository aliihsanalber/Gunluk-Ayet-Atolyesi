//! `ayet web` — yerel arayüz. Yalnız standart kütüphane: tek dosyalık küçük HTTP sunucusu.
//! Kartlar Rust tarafında çizilir, tarayıcıya PNG olarak gönderilir.

use crate::card::{build_pages, parse_hex, Frame, Renderer, SplitMode, Style_};
use crate::data::{verse_texts, Quran};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

const UI: &str = include_str!("ui.html");

pub fn serve(q: Quran, port: u16) {
    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap_or_else(|e| {
        eprintln!("hata: 127.0.0.1:{port} açılamadı ({e}). Başka bir port için: ayet web --port 8800");
        std::process::exit(1)
    });
    println!("Günün Ayeti Atölyesi → http://localhost:{port}/   (çıkmak için Ctrl+C)");
    let q = Arc::new(q);
    let r = Arc::new(Mutex::new(Renderer::new()));
    for stream in listener.incoming().flatten() {
        let (q, r) = (q.clone(), r.clone());
        std::thread::spawn(move || {
            let _ = handle(stream, &q, &r);
        });
    }
}

fn respond(s: &mut TcpStream, code: u16, ctype: &str, body: &[u8]) -> std::io::Result<()> {
    let text = match code {
        200 => "OK",
        400 => "Bad Request",
        _ => "Not Found",
    };
    write!(s, "HTTP/1.1 {code} {text}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n", body.len())?;
    s.write_all(body)
}

fn handle(mut stream: TcpStream, q: &Quran, r: &Mutex<Renderer>) -> std::io::Result<()> {
    let mut rd = BufReader::new(stream.try_clone()?);
    let mut first = String::new();
    rd.read_line(&mut first)?;
    let mut it = first.split_whitespace();
    let (method, path) = (it.next().unwrap_or("").to_string(), it.next().unwrap_or("").to_string());
    let mut len = 0usize;
    loop {
        let mut h = String::new();
        rd.read_line(&mut h)?;
        if h.trim().is_empty() {
            break;
        }
        if let Some((k, v)) = h.split_once(':') {
            if k.eq_ignore_ascii_case("content-length") {
                len = v.trim().parse().unwrap_or(0);
            }
        }
    }
    if len > 4 << 20 {
        return respond(&mut stream, 400, "text/plain", b"too large");
    }
    let mut body = vec![0u8; len];
    rd.read_exact(&mut body)?;
    let req: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);

    let json_ok = |v: Value| v.to_string().into_bytes();
    match (method.as_str(), path.as_str()) {
        ("GET", "/") => respond(&mut stream, 200, "text/html; charset=utf-8", UI.as_bytes()),
        ("GET", "/api/meta") => {
            let meals: Vec<Value> = q.meals.iter().map(|(k, m)| json!({"key": k, "name": m.name})).collect();
            let v = json!({"names": q.names, "counts": q.counts, "meals": meals, "default": q.default_meal, "picks": q.picks});
            respond(&mut stream, 200, "application/json", &json_ok(v))
        }
        ("POST", "/api/verses") => match parse_sel(q, &req) {
            Ok((meal, s, a1, a2)) => {
                let v = json!({"label": q.ref_label(s, a1, a2), "texts": verse_texts(q, &meal, s, a1, a2)});
                respond(&mut stream, 200, "application/json", &json_ok(v))
            }
            Err(e) => respond(&mut stream, 400, "application/json", &json_ok(json!({"error": e}))),
        },
        ("POST", "/api/render") => match render(q, r, &req) {
            Ok(v) => respond(&mut stream, 200, "application/json", &json_ok(v)),
            Err(e) => respond(&mut stream, 400, "application/json", &json_ok(json!({"error": e}))),
        },
        _ => respond(&mut stream, 404, "text/plain", b"yok"),
    }
}

fn parse_sel(q: &Quran, req: &Value) -> Result<(String, usize, usize, usize), String> {
    let num = |k: &str| req[k].as_u64().map(|n| n as usize).ok_or(format!("'{k}' eksik"));
    let meal = req["meal"].as_str().unwrap_or(&q.default_meal).to_string();
    if !q.meals.contains_key(&meal) {
        return Err("bilinmeyen meâl".into());
    }
    let s = num("sure")?;
    if !(1..=114).contains(&s) {
        return Err("sûre 1-114 olmalı".into());
    }
    let (a1, a2) = (num("a1")?, num("a2")?);
    let n = q.counts[s - 1];
    if a1 < 1 || a1 > a2 || a2 > n {
        return Err(format!("{} sûresinde ayet aralığı 1-{n} olmalı", q.sure_name(s)));
    }
    Ok((meal, s, a1, a2))
}

fn render(q: &Quran, r: &Mutex<Renderer>, req: &Value) -> Result<Value, String> {
    let (meal, s, a1, a2) = parse_sel(q, req)?;
    let mut texts = verse_texts(q, &meal, s, a1, a2);
    if let Some(arr) = req["texts"].as_array() {
        for (i, t) in arr.iter().enumerate().take(texts.len()) {
            if let Some(t) = t.as_str().filter(|t| !t.trim().is_empty()) {
                texts[i] = t.to_string();
            }
        }
    }
    let mut st = Style_::default();
    st.frame = match req["frame"].as_str().unwrap_or("suslu") {
        "sade" => Frame::Simple,
        "yok" => Frame::None,
        _ => Frame::Ornate,
    };
    if let Some(c) = req["bg"].as_str().and_then(parse_hex) {
        st.bg = c;
    }
    if let Some(c) = req["ink"].as_str().and_then(parse_hex) {
        st.ink = c;
    }
    if let Some(c) = req["baslik"].as_str().and_then(parse_hex) {
        st.src_ink = c;
        st.rule = c;
    }
    let mode = match req["split"].as_str().unwrap_or("otomatik") {
        "kapali" => SplitMode::Off,
        "zorla" => SplitMode::Force,
        _ => SplitMode::Auto,
    };
    let besmele = req["besmele"].as_bool().unwrap_or(true);

    let mut r = r.lock().map_err(|_| "dahili hata".to_string())?;
    let pages = build_pages(&mut r, q, &meal, &texts, s, a1, a2, besmele, mode, st.frame);
    let mut out = Vec::new();
    for pg in &pages {
        let png = r.render(pg, &st).encode_png().map_err(|e| e.to_string())?;
        out.push(json!({"src": pg.src, "png": b64(&png)}));
    }
    Ok(json!({"pages": out}))
}

fn b64(d: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut o = String::with_capacity(d.len().div_ceil(3) * 4);
    for c in d.chunks(3) {
        let n = (c[0] as u32) << 16 | (*c.get(1).unwrap_or(&0) as u32) << 8 | *c.get(2).unwrap_or(&0) as u32;
        o.push(T[(n >> 18) as usize & 63] as char);
        o.push(T[(n >> 12) as usize & 63] as char);
        o.push(if c.len() > 1 { T[(n >> 6) as usize & 63] as char } else { '=' });
        o.push(if c.len() > 2 { T[n as usize & 63] as char } else { '=' });
    }
    o
}

#[cfg(test)]
mod tests {
    #[test]
    fn base64() {
        assert_eq!(super::b64(b"Man"), "TWFu");
        assert_eq!(super::b64(b"Ma"), "TWE=");
        assert_eq!(super::b64(b"M"), "TQ==");
    }
}
