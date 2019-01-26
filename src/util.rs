use quick_xml::Reader;
use quick_xml::events::Event as xmlEvent;
use std::collections::HashMap;
use std::borrow::Cow;
use std::str::FromStr;

pub fn read_spritesheet(filename: String) -> HashMap<String, HashMap<String, i32>> {
    let mut reader = Reader::from_file(filename).unwrap();
    reader.trim_text(true);

    let mut buf = Vec::new();

    let mut t : HashMap<String, HashMap<String, i32>> = HashMap::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(xmlEvent::Start(ref e)) => {
                match e.name() {
                    b"TextureAtlas" => println!("attributes values: {:?}",
                                                e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()),
                    _ => (),
                }
            },
            Ok(xmlEvent::Empty(d)) => {

                let mut map_entry: HashMap<String, i32> = HashMap::new();

                //println!("{:?}", d.name());

                let mut name: String = String::new();

                for i in d.attributes() {

                    let attr = i.expect("Couldn't grab attribute");

                    let key = String::from_utf8_lossy(attr.key);

                    if key == "name" {

                        let value = match attr.value {
                            Cow::Borrowed(r) => String::from_utf8_lossy(&r),
                            Cow::Owned(_) => break
                        };
                        name = value.to_lowercase()
                    } else {

                        let value = match attr.value {
                            Cow::Borrowed(r) => String::from_utf8_lossy(&r),
                            Cow::Owned(_) => break
                        };

                        map_entry.insert(String::from(key), FromStr::from_str(&value[..]).expect(""));
                    }
                }

                t.insert(name,map_entry);
            },
            Ok(xmlEvent::Eof) => break,
            _ => (),
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    return t;
}