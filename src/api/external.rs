
pub mod billing {
    use std::hash::RandomState;

    use serde_json::{json, Value};
    use reqwest::Client;

    pub async fn request_payload() -> Result<(), String> {
        let site_id = "446007";
        let site_secret = "test_DSkMWKEGSrSDPdUP5wi8QmfELX17ZS8t7-t5S5C1TGs";

        let body = json!({
            "payment_id": "1",
            "amount": {
                "value": "1.00",
                "currency": "RUB",
            },
            "capture": true,
            "confirmation": {
                "type": "redirect",
                "return_url": "http://localhost:8000/billing/callbacks"
            },
            "metadata": {
                "order_id": 1 // TODO
            },
            "description": "pff"
        });

        let request_url = "https://api.yookassa.ru/v3/payments";
        let response = Client::new()
            .post(request_url)
            .basic_auth(site_id.clone(), Some(site_secret.clone()))
            .header("Idempotence-Key", "12312312s")
            .json(&body)
            .send().await;

        println!("text: {response:?}");
        println!("dfdf {}", response.as_ref().is_ok_and(|r| r.status() == 200 ));

        match response {
            Ok(res) => {
                let bodyy: Result<Value, reqwest::Error> = res.json::<Value>().await;
                println!("dkfkdkfd {bodyy:?}");

                ();
            },
            Err(_) => (),
        };

        return Ok(());
    }
}
