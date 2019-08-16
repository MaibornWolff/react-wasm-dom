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

All diese Libraries haben die Gemeinsamkeit, dass man gezwungenermaßen in der Programmiersprache seine GUI programmieren muss, in der auch die Library geschrieben ist.
Für Entwickler, die diese Sprache beherrschen ist das vorteilhaft, da diese dadurch befähigt werden auch ihre eigenen Web-Apps zu entwickeln in ihrer bevorzugten Programmiersprache.
Was ich persönlich vermisse ist eine Virtual DOM Library, die zwar in WASM läuft, welche aber eine JavaScript API bereit stellt.
Womöglich extra eine andere Programmiersprache zu lernen um die Performancegewinne von WASM zu erhalten, klingt nach sehr viel Aufwand und es gibt schon extrem gute Lösungen in JavaScript, mit denen es einfach ist eine Virtual DOM zu beschreiben.

## JSX

JSX ist eine speziell für React entwickelte Syntax um die Virtual DOM zu repräsentieren.
Mit Hilfe von Babel wird diese dann in pures JavaScript umgewandelt, was man auch transpilieren nennt.
Die lose Kopplung von JSX an React ermöglicht es dieses nicht nur für React zu verwenden.
Man kann seine eigene Virtual DOM mit JSX implementieren, das bekannteste Beispiel ist [Preact](https://preactjs.com/).
Preact versucht so gut es geht ein Drop-In-Replacement für React zu sein.
Das klappt mehr oder weniger gut und hängt vor allem mit der Mächtigkeit von React zusammen.

Um zu verstehen wie man JSX verwenden kann, muss man zunächst wissen was mit dem JSX im Transpilierungsschritt geschieht.
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

### JSX Transpilation

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
  entry: "./src/js/index.tsx",
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
In der `index.ts`-Datei deklarieren wir "h" folgendermaßen:

```ts
function h<P, S>(
  type: Component<P, S>,
  props: P,
  ...children: JSX.Element[]
): JSX.Element {
  return {
    type,
    props: props || {},
    children
  };
}
```

Ein erneutes Ausführen von `npm run start` ist weiterhin erfolgreich, diesmal aber powered by Babel.

Um zu verstehen was Babel mit JSX macht und wie wir das ganze zu WASM bringen können, müssen wir uns einmal anschauen wie unser Code transpiliert wird. Dafür gibt es [hier](https://babeljs.io/repl/#?babili=false&browsers=&build=&builtIns=false&spec=false&loose=false&code_lz=PQKhAIAECsGcA9wAtwmAKHQMwK4DsBjAFwEsB7PZACiIE8AHAUwBpx6AnM-2VgOn4JISAGwAm7RngCU4AN7pw4CURztK8xYrpNmCzRy6wAXG07dwAHwtyAvrs3hBI8ZL02A3OhuYCFWEXAAQXp6cABecCoDbhkwgD5wAB5REgA3ONlTQ15SImFGcBtE4BT0z3RE4NDc_LCAcgAJRmFhMnAAdTJ2MTrwYDj3IA&debug=false&forceAllTransforms=false&shippedProposals=false&circleciRepo=&evaluate=false&fileSize=false&timeTravel=false&sourceType=module&lineWrap=true&presets=react%2Cstage-2%2Ctypescript&prettier=false&targets=&version=7.4.3&externalPlugins=) ein minimales Beispiel:

```tsx
/** @jsx h */
function h(type, props, ...children) {
  return {
    type,
    props: props || {},
    children
  };
}

const App = props => <div>{props.title}</div>;

<App title="Hello World" />;
```

wird zu

```jsx
/** @jsx h */
function h(type, props, ...children) {
  return {
    type,
    props: props || {},
    children
  };
}

const App = props => h("div", null, props.title);

h(App, {
  title: "Hello World"
});
```

Jedes JSX-Element besteht aus einem `type`, `props` und `children`.

`type` ist im Falle einer Component entweder eine Class oder eine Function.
Wenn hingegen ein IntrinsicElement gerendert wird, so ist es ein String, wie z.B. bei "div".
`props` ist das Property-Objekt und `children` sind die von den JSX-Tags umschlossenen Elemente.

Letztendlich wird aus dem JSX-Element also einfach nur ein Aufruf aus "h" generiert und dessen return-Value können wir an WASM übergeben um es zu rendern.

### Bootstrapping

[wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) ist eine Library, mit der wir JavaScript und Rust einfach miteinander verknüpfen können. Durch prozedurale Macro-Annotationen in Rust werden automatisch JavaScript exports oder imports erzeugt.

Zunächst werden wir ein JSX Struct definieren, welches von JavaScript zu Rust gegeben werden kann wie folgt:

```rust
#[wasm_bindgen]
extern "C" {
    pub type Jsx;

    #[wasm_bindgen(method, getter)]
    pub fn props(this: &Jsx) -> js_sys::Object;

    #[wasm_bindgen(method, getter)]
    pub fn children(this: &Jsx) -> js_sys::Array;

    #[wasm_bindgen(method, getter, js_name=type)]
    pub fn jsx_type(this: &Jsx) -> JsValue;
}
```

Der Typ Jsx ist genau der Rückgabewert, den man von der h-Funktion erhält. Mit diesem Wert bootstrappen wir unsere Anwendung innerhalb von Rust, indem wir eine `render`-Funktion in Rust definieren und exportieren.

```rust
#[wasm_bindgen]
pub fn render(jsx: &Jsx) -> Result<(), JsValue> {
    let element = render_jsx(jsx)?;
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    body.append_child(&element)?;
    Ok(())
}

fn render_jsx(jsx: &Jsx) -> Result<web_sys::Element, JsValue> {
    // ...
}
```

JsxType ist ein Enum um jegliche Arten von rendering zu ermöglichen.
React unterstützt zum einen Klassenkomponenten aber auch funktionale Komponenten.
IntrinsicElements sind die Elemente, die im DOM gerendert werden können und als String übergeben werden.

```rust
pub enum JsxType {
    Component(js_sys::Function),
    Functional(js_sys::Function),
    Intrinsic(String),
}
```

Durch die Annotation `#[wasm_bindgen]` wird die render-Funktion in JavaScript exportiert und unsere App kann somit gebootstrapped werden:

```js
// index.tsx

import("../../pkg/react_wasm").then(module => {
  try {
    module.render(<App />);
  } catch (err) {
    console.error(err);
  }
});
```

### DOM Rendering

Nun kommen wir zum eigentlichem Teil unserer Anwendung.
Den aus JavaScript erhaltenen JSX-Tree müssen wir nun in der DOM rendern.
Die Funktion `render_jsx` liefert hierfür ein Element, wenn alles gut läuft, ansonsten bekommen wir ein Fehlerobjekt, welches in die Konsole geschrieben wird.

```rust
fn render_jsx(jsx: &Jsx) -> Result<web_sys::Element, JsValue> {
    match jsx.jsx_type().try_into()? {
        JsxType::Component(constructor) => {
            let component: Component = // ...
            render_jsx(&component.render())
        }
        JsxType::Functional(function) => {
            let jsx: Jsx = // ...
            render_jsx(&jsx)
        }
        JsxType::Intrinsic(intrinsic) => {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let element = document.create_element(&intrinsic)?;

            jsx.children()
                .for_each(&mut |val: JsValue, _index, _array| {
                  // Hier werden die HTMLElemente oder Strings in das DOM gerendert.
                });
            Ok(element)
        }
    }
}

impl TryInto<JsxType> for JsValue {
    type Error = JsValue;

    fn try_into(self) -> Result<JsxType, Self::Error> {
        if self.is_function() {
            let function: js_sys::Function = self.unchecked_into();
            if Jsx::is_constructor(&function) {
                Ok(JsxType::Component(function))
            } else {
                Ok(JsxType::Functional(function))
            }
        } else if let Some(intrinsic) = self.as_string() {
            Ok(JsxType::Intrinsic(intrinsic))
        } else {
            Err("bad jsx value".into())
        }
    }
}

impl Jsx {
    fn is_constructor(function: &js_sys::Function) -> bool {
        match js_sys::Reflect::construct(function, &js_sys::Array::new()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
```

Wer noch nicht viel mit Rust gearbeitet hat, dem wird vorheriger Code etwas fremdlich erscheinen.
Der `?`-Operator ist eine elegante Möglichkeit den Wert eines Results zu lesen oder im Fehlerfall den Fehler weiter nach oben zu reichen.

`TryInto` ist ein Trait, den man zur Konvertierung zwischen Datentypen verwenden kann.
Hier wird versucht einen beliebigen Wert aus JavaScript in den passenden JsxType zu konvertieren um darauf Pattern Matching anzuwenden.

Durch JavaScript Reflection testen wir, ob es sich bei der JavaScript-Funktion um einen Konstruktor handelt oder nicht.
