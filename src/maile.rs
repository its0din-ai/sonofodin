use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};


pub async fn send(){
    let message = MessageBuilder::new()
        .from(("encrypt0r", "encrypt0r.x86@gmail.com"))
        .to(vec![
            ("RECV_NAME", "RECV_MAIL"),
        ])
        .subject("Hi!")
        .html_body("<!doctype html>
        <html lang='en'>
        <head>
            <meta charset='utf-8'>
            <meta name='viewport' content='width=device-width, initial-scale=1'>
            <title>Bootstrap demo</title>
            <link href='https://cdn.jsdelivr.net/npm/bootstrap@5.3.1/dist/css/bootstrap.min.css' rel='stylesheet'
                integrity='sha384-4bw+/aepP/YC94hEpVNVgiZdgIC5+VKNBQNGCHeKRQN+PtmoHDEXuppvnDJzQIu9' crossorigin='anonymous'>
        </head>
        <body>
            <div class='container-fluid text-center mt-5'>
                <div class='card text-bg-dark mx-auto my-auto w-50'>
                    <div class='card-body'>
                      <h5 class='card-title'>SON OF ODIN</h5>
                      <h6 class='card-subtitle mb-2 text-warning'>Discord BOT Mailer</h6>
                      <p class='card-text'>Email ini dikirim melalui Discord Bot menggunakan Rust</p>
                      <a type='button' href='https://github.com/its0din-ai/sonofodin' class='btn btn-sm btn-outline-warning'>Verify</a>
                    </div>
                  </div>
            </div>
        </body>
        </html>
        ")
        .text_body("Son Of Odin!");

    SmtpClientBuilder::new("smtp.gmail.com", 587)
        .implicit_tls(false)
        .credentials(("encrypt0r.x86@gmail.com", "APP_PASSWORD"))
        .connect()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();
}
