use std::error::Error;

extern crate csv;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize, PartialEq)]
struct BitwardenRecord {
    folder: String,
    favorite: String,
    #[serde(rename = "type")]
    record_type: String,
    name: String,
    notes: String,
    login_uri: String,
    login_username: String,
    login_password: String,
}

#[derive(Debug, Serialize, PartialEq)]
struct LastPassRecord {
    url: String,
    username: String,
    password: String,
    extra: String,
    name: String,
    grouping: String,
    fav: String,
}

pub fn convert_file(bw_csv_path: &str, lp_csv_path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(bw_csv_path)?;
    let mut wtr = csv::Writer::from_path(lp_csv_path)?;
    for bw_result in rdr.deserialize() {
        let bw: BitwardenRecord = bw_result?;
        let lp: LastPassRecord = convert_record(bw);
        wtr.serialize(lp)?;
    }
    Ok(())
}

fn convert_record(bw: BitwardenRecord) -> LastPassRecord {
    let bw_login_uri = bw.login_uri.as_str();
    LastPassRecord {
        url: match bw.record_type.as_str() {
            "login" => bw_login_uri.to_string(),
            _ => "http://sn".to_string(),
        },
        username: bw.login_username,
        password: bw.login_password,
        extra: bw.notes,
        name: bw.name,
        grouping: bw.folder,
        fav: bw.favorite,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_login_record() {
        assert_eq!(convert_record(test_bw_record()), test_lp_record());
    }

    #[test]
    fn convert_note_record() {
        assert_eq!(convert_record(
            BitwardenRecord {
                record_type: "note".to_string(),
                ..test_bw_record()
            }), LastPassRecord {
                url: "http://sn".to_string(),
                ..test_lp_record()
            });
    }

    fn test_bw_record() -> BitwardenRecord {
        BitwardenRecord {
            folder: "folder".to_string(),
            favorite: "0".to_string(),
            record_type: "login".to_string(),
            name: "name".to_string(),
            notes: "notes".to_string(),
            login_uri: "https://example.org/login".to_string(),
            login_username: "username".to_string(),
            login_password: "password".to_string(),
        }
    }

    fn test_lp_record() -> LastPassRecord {
        LastPassRecord {
            url: "https://example.org/login".to_string(),
            username: "username".to_string(),
            password: "password".to_string(),
            extra: "notes".to_string(),
            name: "name".to_string(),
            grouping: "folder".to_string(),
            fav: "0".to_string(),
        }
    }
}