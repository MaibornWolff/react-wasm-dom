# Yet another WebAssembly Virtual DOM

WebAssembly (kurz WASM) wird immer populärer und es tauchen immer mehr Möglichkeiten auf wie man schon jetzt ganze Webseiten damit bauen kann.

Die wohl bekannteste Form davon nennt sich Virtual DOM und wird so z.B. in React verwendet.
Das Prinzip ist eigentlich relativ simpel.
Anstatt direkt HTML zu schreiben hat man stattdessen eine andere Repräsentation der DOM in einer anderen Programmiersprache und potentiell anderer Syntax.
Die Virtual DOM Library entscheidet dann wie diese Repräsentation in die echte DOM transferiert wird, also in HTML.
Das hat z.B. den Vorteil, dass man dynamisch Teile des generierten HTML ersetzen kann ohne die Seite neu zu laden, das nennt man dann Single Page Application (SPA) und gibt einem das Gefühl als würde man eine native App bedienen.

Damit die Virutal DOM Library im Browser läuft, muss sie entweder zu JavaScript oder WASM kompiliert werden, da nur diese beiden Sprachen im Browser laufen.
Ich möchte in diesem Blogartikel die Möglichkeit aufzeigen dies in Rust und mit der bereits existierenden JSX-Syntax zu tun.

## Virtual DOMs in WASM

Viele Programmiersprachen bieten die Möglichkeit an eine Virtual DOM Library mit Hilfe von WASM zu verwenden.
Eine Übersicht findet man [hier](https://github.com/mbasso/awesome-wasm#web-frameworks-libraries).

All diese Sprachen sind allerdings darauf fokusiert ihre eigene API anzubieten.
Das hat den Nachteil, dass man diese erst lernen muss und auch, dass man die Programmiersprache beherrscht.

JSX hingegen ist ein weit verbreitetes Format und JavaScript ist eine der meist genutzten Programmiersprachen.
Wenn man eine Virtual DOM Library bauen würde mit der exakt gleichen Syntax wie React würde man so viel mehr Leute erreichen.
Dass im Hintergrund alles in WASM läuft, muss den Nutzer ja nicht interessieren.
Wichtig ist vor allem, dass es funktioniert und dass man damit produktiv ist.

## JSX

JSX ist eine speziell für React entwickelte Syntax um die Virtual DOM zu repräsentieren.
Mit Hilfe von Babel wird diese dann in pures JavaScript umgewandelt, was man auch transpilieren nennt.
Die lose Kopplung von JSX an React ermöglicht es dieses nicht nur für React zu verwenden.
Man kann seine eigene Virtual DOM mit JSX implementieren, das bekannteste Beispiel ist [Preact](https://preactjs.com/).
Preact versucht so gut es geht ein Drop-In-Replacement für React zu sein.
Das klappt mehr oder weniger gut und hängt vor allem mit der Mächtigkeit von React zusammen.

Um zu verstehen wie man JSX für sich verwenden kann, muss man zunächst wissen was mit dem JSX im Transpilierungsschritt geschieht.
Babel bietet hier ein [Plugin](https://babeljs.io/docs/en/babel-plugin-transform-react-jsx) an.
Auf der Seite sieht man auch mehrere Beispiele wie JSX zu JavaScript transpiliert wird.
Im Standardfall von React wird hier einfach nur die `React.createElement`-Funktion aufgerufen.
Man kann dies aber auch durch eine Konfiguration ändern, sodass eine selbst implementierte Funktion stattdessen aufgerufen wird und genau dies machen wir uns zu nutzen.

## Getting Started

Mit diesem Hintergrundwissen werden wir nun versuchen ein Projekt aufzusetzen und "quasi React in Rust neu implementieren".
Das ist allerdings viel mehr gesagt als getan.
Viel mehr möchte ich zeigen, dass dies möglich wäre.
Wer Lust hat, kann sich gerne austoben, denn aktuell muss WASM noch JavaScript-Funktionen aufrufen um die DOM zu manipulieren.
Solange das [Web IDL Bindings Proposal](https://github.com/WebAssembly/webidl-bindings) (vorher bekannt als Host Bindings Proposal) noch nicht in den Browsern angekommen ist, wird die Performance solch einer WASM Virtual DOM Library vergleichbar sein mit JavaScript.

### Start mit wasm-pack

[wasm-pack](https://rustwasm.github.io/wasm-pack/) bietet uns einen schnellen Einstieg in Rust + WASM, also werden wir dies verwenden.

Nachdem wir wasm-pack installiert haben, können wir unsere Library erstellen:

```bash
$ npm init rust-webpack react-wasm
```

Wenn man zunächst in die Git History schaut, findet man dort alles mögliche von dem von uns kopiertem Projekt.
Eigentlich wollen wir das nicht, deswegen überschreiben wir die Git History mit unserem initialem Commit:

```bash
$ rm -rf .git
$ git init
$ git add .
$ git commit -m "initial commit"
```

Mit Hilfe von `npm run start` können wir unser Projekt kompilieren und auf Port 8080 begutachten.
Die erste Kompilierung dauert ziemlich lange, also muss man etwas warten.

Als nächstes wollen wir JSX einbinden, also müssen wir alle Dependencies installieren um dies zu tun.
TypeScript werden wir auch noch verwenden:

```bash
$ npm i -D @babel/core @babel/plugin-syntax-dynamic-import @babel/plugin-transform-react-jsx @babel/plugin-transform-typescript @babel/polyfill @babel/preset-env babel-loader typescript
```

Jetzt können wir unsere Webpack Config updaten.
Hier steckt schon ziemlich viel Wissen drin.
Die einzelnen Blöcke sind in den jeweiligen Abschnitten erklärt mit den Kommentaren.

```js
  ...
  entry: "./js/index.ts",
  ...
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        use: [
          {
            loader: "babel-loader",
            query: {
              babelrc: false,
              presets: [
                [
                  "@babel/env",
                  {
                    // Dies dient dafür genau die Polyfills bereitzustellen,
                    // die von unserer Library verwendet werden für genau die Browser,
                    // die WASM ausführen können.
                    targets: {
                      browsers: [
                        "edge >= 17",
                        "ff >= 61",
                        "chrome >= 63",
                        "safari >= 11.1"
                      ]
                    },
                    useBuiltIns: "usage",
                    modules: false
                  }
                ]
              ],
              plugins: [
                [
                  "@babel/plugin-transform-typescript",
                  {
                    // Preserve custom JSX Pragma
                    isTSX: true,
                    jsxPragma: "h"
                  }
                ],
                // WASM-Dateien müssen dynamisch geladen werden.
                // Sie können nicht im initialem Entrypoint unserer App enthalten sein
                "@babel/plugin-syntax-dynamic-import",
                [
                  "@babel/plugin-transform-react-jsx",
                  {
                    // Preserve JSX/TSX
                    pragma: "h",
                    pragmaFrag: "Fragment"
                  }
                ]
              ]
            }
          }
        ]
      }
    ]
  }
```

Mit Pragma ist unsere Custom `React.createElement`-Funktion gemeint.
Damit Babel hier nicht den Standard verwendet, müssen wir konfigurieren wie wir diese Funktion bei uns nennen, "h" in diesem Fall.

Ein erneutes Ausführen von `npm run start` ist weiterhin erfolgreich, diesmal aber powered by Babel.
