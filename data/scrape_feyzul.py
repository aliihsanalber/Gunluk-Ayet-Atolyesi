import sys, re, json
import urllib.request

BASE = "https://www.feyzulfurkan.com"
HEADERS = {'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'}

# Fetch the surah list page
req = urllib.request.Request(f"{BASE}/sureler/", headers=HEADERS)
with urllib.request.urlopen(req, timeout=15) as f:
    html = f.read().decode('utf-8', errors='replace')

surah_links = re.findall(r'href=["\'](/sureler/[^"\']+)["\']', html)
seen = set()
surah_urls = []
for link in surah_links:
    if link not in seen:
        seen.add(link)
        surah_urls.append(link)

print(f"Found {len(surah_urls)} surah links")

# Try first surah
url = BASE + surah_urls[0]
req = urllib.request.Request(url, headers=HEADERS)
with urllib.request.urlopen(req, timeout=15) as f:
    html = f.read().decode('utf-8', errors='replace')

# Find the main content area
content_match = re.search(r'entry-content[^>]*>(.*?)(?:</article>|</div>\s*<footer)', html, re.DOTALL)
if content_match:
    content = content_match.group(1)
    # Extract verses
    ayet_blocks = re.findall(r'<strong>\s*(\d+)\s*\.\s*[Aa]yet\s*</strong>\s*</p>\s*<p[^>]*>(.*?)</p>', content, re.DOTALL)
    print(f"Found {len(ayet_blocks)} verses in {surah_urls[0]}")
    for num, txt in ayet_blocks[:3]:
        clean = re.sub(r'<[^>]+>', '', txt).strip()
        print(f"  {num}: {clean[:120]}")
else:
    print("No entry-content found. Looking for patterns...")
    ayets = re.findall(r'<strong>\s*(\d+)\s*\.\s*[Aa]yet\s*</strong>', html)
    print(f"Found {len(ayets)} 'Ayet' patterns")
    # Try another pattern
    ayets2 = re.findall(r'(\d+)\.\s*Ayet', html)
    print(f"Found {len(ayets2)} 'Ayet' mentions")
    if len(ayets2) > 0:
        print(f"First 5: {ayets2[:5]}")