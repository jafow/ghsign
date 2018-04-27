extern crate reqwest;

pub struct GhSign {
    pub username: String
}

impl GhSign {

    fn fetch_key (self) {
        let base_url = "https://github.com/";
        let request_url = format!("{}{}.keys", base_url, self.username);

        let res = reqwest::get(request_url);
        return res;
    }

    pub fn sign (self) {
        let pk = self.fetch_key();
        println!("pk : {:?}", pk);
    }
}
