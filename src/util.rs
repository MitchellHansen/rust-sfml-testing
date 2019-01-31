use quick_xml::Reader;
use quick_xml::events::Event as xmlEvent;
use std::collections::HashMap;
use std::borrow::Cow;
use std::str::FromStr;
use sfml::graphics::IntRect;

pub fn read_spritesheet(filename: String) -> HashMap<String, HashMap<String, i32>> {

    let mut reader = Reader::from_file(filename).unwrap();
    reader.trim_text(true);

    let mut buf = Vec::new();

    let mut t : HashMap<String, HashMap<String, i32>> = HashMap::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(xmlEvent::Start(ref e)) => {
                match e.name() {
                    _ => (),
                }
            },
            Ok(xmlEvent::Empty(d)) => {

                let mut map_entry: HashMap<String, i32> = HashMap::new();

                let mut name: String = String::new();

                for i in d.attributes() {

                    let attr = i.expect("Couldn't grab attribute");

                    let key = String::from_utf8_lossy(attr.key);

                    if key == "name" {

                        let value = match attr.value {
                            Cow::Borrowed(r) => String::from_utf8_lossy(&r),
                            Cow::Owned(_) => break
                        };
                        name = value.to_string();
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

pub fn grab_sheet_rec(spritename: String, spritesheet: &HashMap<String, HashMap<String, i32>>) -> IntRect {

    let block_desc = spritesheet.get(&spritename).expect("Can't load sprite");
    IntRect::new(
        *block_desc.get("x").unwrap(),
        *block_desc.get("y").unwrap(),
        *block_desc.get("width").unwrap(),
        *block_desc.get("height").unwrap()
    )
}
