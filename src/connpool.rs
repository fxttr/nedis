use futures::future::join_all;

pub struct Connpool {
    target: String,
    verbose: bool
}

impl Connpool {
    pub fn new(target: &str, verbose: bool) -> Self {
        Self {
            target: target.to_owned(),
            verbose
        }
    }

    #[tokio::main]
    pub async fn bulk(&self, dirs: Vec<String>) {
        let consumers: Vec<_> = dirs.into_iter().map(|x| self.send(x)).collect();
        join_all(consumers).await;
    }

    async fn send(&self, dir: String) {
        let url = self.target.clone() + "/" + &dir;
        let resp = reqwest::get(&url).await.unwrap().status();

        if resp == 200 {
            println!("[+] {} => 200", &url);
        } else if self.verbose {
            println!("[-] {} => {}", &url, resp);
        }
    }
}