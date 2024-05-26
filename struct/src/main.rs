#[derive(Debug, PartialEq, Eq)]
enum Lang {
  English,
  Spanish,
  Chinese,
  Texan,
  Manc,
  French
}

struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
  let mut v :Vec<Greeting> = Vec::new();

  let g : Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Manc, message: String::from("Y'alright WasmEdge?") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::French, message: String::from("Bonjour WasmEdge!") };
  v.push(g);


  for e in &v {
    println!("{:?} {}", e.lang, e.message);
  }

  // Now attempt to lookup greeting for specific language
  let lookup = Lang::Manc;
  let greeting: &Greeting = & v.into_iter().find(|x| x.lang == lookup).unwrap();
  println!("lookup for for {:?} = {:?} {}", lookup, greeting.lang, greeting.message);
  

}
