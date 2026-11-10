/* Günün Ayeti Atölyesi — service worker
   - Sayfa (index.html) önce AĞDAN denenir: yeni sürüm yayınlanınca telefonda hemen görünür;
     internet yoksa önbellekten açılır.
   - meals/*.json meâl dosyaları ilk kullanımda önbelleğe alınır (sonra çevrimdışı çalışır). */
const CACHE = 'ayet-atolyesi-v2';
const SHELL = ['.', 'index.html', 'manifest.json', 'icon-192.png', 'icon-512.png'];

self.addEventListener('install', (e) => {
  e.waitUntil(caches.open(CACHE).then((c) => c.addAll(SHELL)).then(() => self.skipWaiting()));
});

self.addEventListener('activate', (e) => {
  e.waitUntil(
    caches.keys()
      .then((ks) => Promise.all(ks.filter((k) => k !== CACHE).map((k) => caches.delete(k))))
      .then(() => self.clients.claim())
  );
});

self.addEventListener('fetch', (e) => {
  const req = e.request;
  if (req.method !== 'GET') return;
  const url = new URL(req.url);
  if (url.origin !== location.origin) return; /* Google Fonts vb. tarayıcıya bırakılır */
  e.respondWith(
    fetch(req)
      .then((res) => {
        if (res && res.ok) { const copy = res.clone(); caches.open(CACHE).then((c) => c.put(req, copy)); }
        return res;
      })
      .catch(() => caches.match(req).then((r) => r || caches.match('index.html')))
  );
});
