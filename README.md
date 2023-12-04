# Pobieranie zależności

Mogą zająć nawet 4gb więc uwaga

`npm install`

# Uruchamianie dev

Wersja dev ma niezoptymalizowany kod rust i jest bardzo wolna, szczególnie przy wyliczaniu ROC i AUC

`npm run tauri dev`

# Budowanie

Stworzy w katalogu `\src-tauri\target` foldery z binarkami i instalatorami msi na windowsa. Odpalić można również bezpośrednio `src-tauri\target\release\ZadanieED.exe`

`npm run tauri build`