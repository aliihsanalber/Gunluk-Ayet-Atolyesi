"""kuranmeali.com AyetKarsilastirma.php sayfalarindan tum meal metinlerini ceker.
Kullanim: python3 data/scrape_kuranmeali.py <onbellek.json>
Cikti: {anahtar: [[sure1 ayetleri], ...]} ham onbellek (tum meâller)."""
import sys, re, json, html, urllib.request, concurrent.futures as cf
base = json.load(open('meals/diyanet.json', encoding='utf-8'))['tr']
counts = [len(s) for s in base]
H = {'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64)'}
pat = re.compile(r"<div id='([a-z0-9]+)\+1'[^>]*>(.*?)</div></div>", re.S)

def fetch(s, a):
    u = f"https://www.kuranmeali.com/AyetKarsilastirma.php?sure={s}&ayet={a}"
    for t in range(5):
        try:
            h = urllib.request.urlopen(urllib.request.Request(u, headers=H), timeout=30).read().decode('utf-8', 'replace')
            out = {}
            for k, body in pat.findall(h):
                if k == 'arapca': continue
                x = html.unescape(re.sub(r'<[^>]+>', '', body))
                out[k] = re.sub(r'\s+', ' ', x).strip()
            if out: return (s, a, out)
        except Exception as e:
            pass
    raise RuntimeError(f"{s}:{a}")

jobs = [(s + 1, a + 1) for s, c in enumerate(counts) for a in range(c)]
res = {}
with cf.ThreadPoolExecutor(12) as ex:
    for n, (s, a, out) in enumerate(ex.map(lambda j: fetch(*j), jobs)):
        for k, v in out.items():
            res.setdefault(k, [[''] * c for c in counts])[s - 1][a - 1] = v
        if n % 500 == 0: print(n, len(jobs), file=sys.stderr, flush=True)
json.dump(res, open(sys.argv[1], 'w', encoding='utf-8'), ensure_ascii=False)
