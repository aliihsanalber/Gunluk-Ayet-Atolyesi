/**
 * Yerel önizleme sunucusu — dist/ klasörünü sunar.
 *   node build.js && node serve.js
 *   tarayıcı: http://localhost:8791/
 */
const http = require('http'), fs = require('fs'), path = require('path');
const root = path.join(__dirname, 'dist');
const PORT = process.env.PORT || 8791;

http.createServer((req, res) => {
  let p = decodeURIComponent(req.url.split('?')[0]);
  if (p === '/') p = '/gunun-ayeti-atolyesi.html';
  const f = path.join(root, p);
  if (!f.startsWith(root) || !fs.existsSync(f)) { res.writeHead(404); return res.end('bulunamadı'); }
  const ct = { '.html': 'text/html; charset=utf-8', '.png': 'image/png', '.otf': 'font/otf', '.json': 'application/json' }[path.extname(f)] || 'application/octet-stream';
  res.writeHead(200, { 'content-type': ct });
  fs.createReadStream(f).pipe(res);
}).listen(PORT, () => console.log('http://localhost:' + PORT + '/'));
