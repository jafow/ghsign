extern crate reqwest;
extern crate curl;

use curl::easy::Easy;

pub struct GhSign {
    pub username: String
}

impl GhSign {

    fn fetch_key (self) -> Vec<u8> {
        let base_url = "https://github.com/";
        let request_url = format!("{}{}.keys", base_url, self.username);

        let mut res = Vec::new();
        let mut easy = Easy::new();

        easy.url(&request_url).unwrap();
        {
            let mut trans = easy.transfer();
            trans.write_function(|data| {
                res.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();

            trans.perform().unwrap();
        }
        res
    }

    pub fn sign (self) {
        let pk = self.fetch_key();
    }
}
