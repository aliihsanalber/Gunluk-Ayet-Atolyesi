// Web sürümünü (repo kökü) Capacitor'ın www klasörüne kopyalar. Önce `node build.js` + index.html güncel olmalı.
const fs = require('fs'), path = require('path');
const root = path.join(__dirname, '..'), out = path.join(__dirname, 'www');
fs.rmSync(out, { recursive: true, force: true }); fs.mkdirSync(out, { recursive: true });
for (const f of ['index.html', 'manifest.json']) fs.copyFileSync(path.join(root, f), path.join(out, f));
for (const f of ['icon-192.png', 'icon-512.png']) fs.copyFileSync(path.join(root, 'assets', f), path.join(out, f));
fs.cpSync(path.join(root, 'ar'), path.join(out, 'ar'), { recursive: true });
fs.cpSync(path.join(root, 'meals'), path.join(out, 'meals'), { recursive: true });
console.log('www hazır:', fs.readdirSync(out).join(', '));
