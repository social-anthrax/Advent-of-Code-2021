use std::io::Write;


use reqwest::header::COOKIE;

pub fn get_task(task_id: u8) -> String {
    match std::fs::read_to_string(format!("src/tasks/task{}.txt", task_id)) {
        Ok(x) => x,
        Err(_) => {
            let request = reqwest::blocking::Client::new()
                .get(format!(
                    "https://adventofcode.com/2021/day/{}/input",
                    task_id
                ))
                .header(COOKIE, "session=<YourSessionCookieHere>")
                .send();

            match request {
                Ok(response) => {
                    let mut file =
                        std::fs::File::create(format!("src/tasks/task{}.txt", task_id)).unwrap();
                    let body = response.text().unwrap();
                    file.write_all(body.as_bytes()).unwrap();
                    body
                }
                Err(_) => panic!("Error in file fetch."),
            }
        }
    }
}
