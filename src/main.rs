pub mod dom;

fn main() {
    let text = dom::text("Hello world!".to_string());
    let text2 = dom::text("Hello world!2".to_string());

    let elem = dom::elem("div".to_string(), dom::AttrMap::new(), vec![text]);
    let elem2 = dom::elem("div2".to_string(), dom::AttrMap::new(), vec![elem, text2]);

    println!("{:#?}", elem2);
}
